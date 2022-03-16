from os import system as run

def main(args):
    run("cargo install sass-rs")
    run("cargo install minifier")

    print("\n\n Setup successfully")

if __name__ == "__main__":
    from sys import argv

    main(sys.argv)