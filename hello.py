def fizz_buzz():
    result = ("FizzBuzz" if i % 15 == 0 else "fizz" if i %
              3 == 0 else "Buzz" if i % 5 == 0 else i for i in range(100))
    for i in result:
        print(i)


def even():
    a = [int(i) for i in range(100)]
    even = filter(lambda x: x % 2 == 0, a)
    for i in even:
        print(i)


def main():
    even()
    fizz_buzz()


if __name__ == '__main__':
    main()
    fizz_buzz()
