package gf256

import "fmt"

// Irreducible polynomial x^8+x^4+x^3+x^2+1 = 0x11d
// Same polynomial used by klauspost / reedsolomon, zfec, and ISA-L

const prime = 0x11d

var (
	expTable [512]byte
	logtable [256]byte
)

func init() {
	x := 1
	for i := 0; i < 255; i++ {
		expTable[i] = byte(x)
		logTable[x] = byte(i)
		x <<= 1
		if x > 0xff {
			x ^= prime // reduce mod irreducible poly
		}
	}
	for i := 255; i < 512; i++ {
		expTable[i] = expTable[i-255]
	}
}

// Add: XOR - no carry in GF(2^8)
func Add(a, b byte) byte { return a ^ b }

// Mul: log-table multiplication, O(1)
func Mul(a, b byte) byte {
	if a == 0 || b == 0 {
		return 0
	}
	return expTable[int(logTable[a])+int(logTable[b])]
}

// Div: multiply by inverse
func Div(a, b byte) (byte, error) {
	if b == 0 {
		return 0, fmt.Errorf("gf256: division by zero")
	}
	if a == 0 {
		return 0, nil
	}
	idx := (255 + int(logTable[a]) - int(logTable[b])) % 255
	return expTable[idx], nil
}

// Pow: a^exp in GF(256)
func Pow(a byte, exp int) byte {
	if a == 0 {
		return 0
	}
	return expTable[(int(logTable[a])*exp)%255]
}
