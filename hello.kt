
fun fizzBuzz(){
    val function = { x: Int ->
        when {
            x % 15 == 0 -> "FizzBuzz"
            x % 5 == 0 -> "Buzz"
            x % 3 == 0 -> "Fizz"
            else -> x.toString()
        }
    }
    (0..100).map(function).forEach{ println(it) }

}

fun evenOdd(){
    val even = { x: Int -> if(x % 2 == 0) "Even" else "Odd" }
    (0..100).map(even).forEach{ println(it) }
}

fun main(args: Array<String>){
    evenOdd()
    fizzBuzz()
}