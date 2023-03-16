fun printArray(array: Array<Int>) {
    println("[")
    for (item in array) {
        println("  " + item + ",")
    }
    println("]")
}

fun sum(numbers: Array<Int>): Int {
    var sum = 0
    for (number in numbers) {
        sum = sum + number
    }
    return sum
}

fun mul(numbers: Array<Int>): Int {
    var mul = 1
    for (number in numbers) {
        mul = mul * number
    }
    return mul
}

fun main() {
    val array = arrayOf(1, 4, 5, 8)
    print("array = ")
    printArray(array)
    println("sum of array = " + sum(array))
    println("mul of array = " + mul(array))
}