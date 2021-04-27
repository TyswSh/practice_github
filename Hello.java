class Hello {
    public static void fizzBuzz(){
        for(int i = 1; i < 101; i++){
            if(i%15 == 0) System.out.println("FizzBuzz");
            else if(i%5 == 0) System.out.println("Buzz");
            else if(i%3 == 0) System.out.println("Fizz");
            else System.out.println(i);
        }
    }

    public static void evenOdd(){
        for(int i = 0; i < 101; i++){
            if(i%2 == 0) System.out.println("even");
            else System.out.println("odd");
        }
    }

    public static void main(String[] args){
        FizzBuzz();
    }
}