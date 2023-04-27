fun size(arr: Array<Int>): Int {
    var size = 0
    for (el in arr) {
        size = size + 1
    }
    return size
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

fun printArray(arr: Array<Int>) {
    for (num in arr) {
        print("" + num + ", ")
    }
    println()
}

fun main() {
    val arr = arrayOf(15, 3, 12, 6, -9, 9, 0)
    print("Before Sorting: ")
    printArray(arr)
    insertionSort(arr)
    print("After Sorting: ")
    printArray(arr)
}