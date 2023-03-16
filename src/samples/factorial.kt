fun factorial(number: Int): Int {
    if (number < 0) {
        return -1
    } else {
        var result = 1
        for (num in 1..number) {
            result = result * number
        }
        return result
    }
}

fun main() {
    for (number in 0..5) {
        println("factorial of " + number + " is " + factorial(number))
    }
}