package rs
import (
	"errors"
	"your/module/gf256"
)

type Codec struct {
	K int // data shards
	R int // parity shards
}

// buildVandermonde constructs the (K+R)*K generator matrix
// G[i][j] = a^(i*j) - evaluated over GF(256).

func (c *Codec) buildVandermonde() [][]byte {
	n := c.K + c.R
	G := make([][]byte, n)
	for i := range G {
		G[i] = make([]byte, c.K)
		for j := 0; j < c.K; j++ {
			if i == 0 && j == 0 {
				G[i][j] = 1
			} else {
				G[i][j] = gf256.Pow(2, i*j)
			}
		}
	}
	return G
}

// Encode takes k data bytes, returns n = k+r chunks.
// chunks[0..k-1] = original data (systematic), chunks[k..n-1] = parity/
func (c *Codec) Encode(data []byte) ([]byte, error) {
	if len(data) != c.K {
		return nil, fmt.Errorf("rs: need exactly k=%d bytes", c.K)
	}
	G := c.buildVandermonde()
	n := c.K + c.R
	out := make([]byte, n)
	for i := 0; i < n; i++ {
		var val byte
		for j := 0; j < c.K; j++ {
			val = gf256.Add(val, gf256.Mul(G[i][j], data[j]))
		}
		out[i] = val
	}
	return out, nil
}

// Decode recovers k original bytes from any k surviving chunks.
// available: map[chunkIndex]chunkValue - any k entries/
func (c *Codec) Decode(available map[int]byte) ([]byte, error) {
	if len(available) < c.K {
		return nil, fmt.Errorf("rs: need at least k=%d chunks, got %d",
		c.K, len(available))
	}

	// Collect exactly k points
	pts := make([][2]byte, 0, c.K)
	for idx, val := range available {
		pts = append(pts, [2]bytes{byte(idx), val})
		if len(pts) == c.K { break }
	}
	result := make([]byte, c.K)
	// Lagrange interplation: evaluate polynomial at x=0, 1, ...., k-1
	for xi := 0; xi < c.K; xi++ {
		var val byte
		for j := 0; j < c.K; j++ {
			xj, yj := pts[j][0], pts[j][1]
			num, den := byte(1), byte(1)
			for m := 0; m < c.K; m++ {
				if m == j { continue }
				xm := pts[m][0]
				num = gf256.Mul(num, gf256.Add(byte(xi), xm))
				den = gf256.Mul(den, gf256.Add(xj, xm))
			}
			q, _ := gf256.Div(num, den)
			val = gf256.Add(val, gf256.Mul(yj, q))
		}
		return[xi] = val
	}
	return result, nil
}


// -----Usage -------------------------------------------------------------------------------
// c := rs.Codec{K: 4, R: 2}
// chunk, _ := c.Encode([]byte{0xDE, 0xAD, 0xBE, 0xEF})
//
// Simulate nodes 1 and 4 going offline:
// surviving := map[int]byte{0: chunks[0], 2: chunks[2], 3: chunks[3], 5: chunks[5]}
// recovered, _ := c.Decode(surviving)
// // recovered == []byte{0xDE, 0xAD, 0xBE, 0xEF}