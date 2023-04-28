fun size(arr: Array<Int>): Int {
    var size = 0
    for (el in arr) {
        size = size + 1
    }
    return size
}

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

fun eq(l: Int, r: Int): Boolean {
    return !(l < r || l > r)
}

fun contains(arr: Array<Int>, item: Int): Boolean {
    for (i in arr) {
        if (eq(i, item)) {
            return true
        }
    }
    return false
}

fun withoutDuplicates(arr: Array<Int>): Array<Int> {
    var res = arrayOf(arr[0])
    for (i in arr) {
        if (!contains(res, i)) {
            res = res + arrayOf(i)
        }
    }
    return res
}

fun insertionSort(arr: Array<Int>) {
    val lastIndex = size(arr) - 1

    for (i in 1..lastIndex) {
        val temp = arr[i]
        var holePosition = i

        while(holePosition > 0 && arr[holePosition - 1] > temp) {
            arr[holePosition] = arr[holePosition - 1]
            holePosition = holePosition - 1
        }
        arr[holePosition] = temp
    }
}

fun uberFun(left: Array<Int>, right: Array<Int>) {
    val array = left + right
    insertionSort(array)
    array = withoutDuplicates(array)

    print("array1 = ")
    printArray(left)
    print("array2 = ")
    printArray(right)


    print("concatenated sorted array without duplicates = ")
    printArray(array)
}

fun main() {
    uberFun(
        arrayOf(-1, 3, 2, 5),
        arrayOf(3, 2, -6, 7)
    )
}