def main():
    a = [int(i) for i in range(1000)]
    even = filter(lambda x: x%2 == 0, a)
    

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
