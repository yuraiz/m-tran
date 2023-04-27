fun size(arr: Array<String>): Int {
    var size = 0
    for (el in arr) {
        size = size + 1
    }
    return size
}

fun insertionSort(arr: Array<String>) {
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

fun printArray(arr: Array<String>) {
    for (num in arr) {
        print("" + num + ", ")
    }
    println()
}

fun main() {
    val arr = sampleStrings()
    println("Before Sorting: ")
    printArray(arr)

    insertionSort(arr)
    
    println("After Sorting: ")
    printArray(arr)
}

fun sampleStrings(): Array<String> {
    return arrayOf(
        "parsing", 
        "bypassing",
        "Ranch",
        "innovative",
        "granular",
        "Producer",
        "Armenia",
        "adapter",
        "Sleek",
        "Future",
        "Officer",
        "Interactions",
        "hack",
        "Bahamian",
        "integrate",
        "United",
        "Spain",
    )
}