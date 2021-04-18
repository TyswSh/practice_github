
def main():
    a = [int(i) for i in range(1000)]
    even = filter(lambda x: x % 2 == 0, a)
    for i in even:
        print(i)


if __name__ == '__main__':
    main()
