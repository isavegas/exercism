package luhn

import (
	"errors"
)

// Valid returns true if it passes a luhn check
func Valid(input string) bool {

	length := len(input)
	count := 1
	spaceCount := 0
	sum := 0
	for index := length - 1; index >= 0; index-- {
		thisRune := rune(input[index])

		if thisRune == ' ' {
			spaceCount++
			continue
		}

		val, err := digitAsInt(thisRune)
		if err != nil {
			return false
		}

		if count%2 == 0 {
			sum += luhnDouble(val)
		} else {
			sum += val
		}
		count++
	}

	return (length-spaceCount) > 1 && (sum%10) == 0
}

func digitAsInt(thisRune rune) (int, error) {
	val := int(thisRune - '0')
	if !(val >= 0 && val <= 9) {
		return 0, errors.New("non space char")
	}
	return val, nil
}

func luhnDouble(val int) int {
	val *= 2
	if val >= 10 {
		val -= 9
	}
	return val
}
