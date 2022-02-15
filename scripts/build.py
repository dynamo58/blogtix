#!/usr/bin/python

# 						READ !
# ------------------------------------------------------
# This script (and all the other ones in this directory)
# 	are designed to be ran from the root directory
#	of this project (the one with the Cargo.toml file)

# Build local files according to specified CLI args

from shutil import copytree, rmtree, make_archive, copy, move
from os import path, remove, mkdir, system as run
from typing import List

def main(args):
    if "--static" in args:
        compile_everything(True, False)

		if path.isfile("static.zip"): remove("static.zip")
		if path.isdir("_temp"): rmtree("_temp")
		run("cargo run --release --bin build_static")

		copy("static/main.min.css", "_temp")
		copy("static/main.min.js", "_temp")
		copy("static/favicon.ico", "_temp")

		copytree("static/avatars", "_temp/avatars")
		copytree("static/thumbnails", "_temp/thumbnails")

		make_archive("static", "zip", "_temp")
        # rmtree("_temp")
    
    # exporting the whole thing to a zip file
	elif "--export" in args:
		compile_everything(True, False)
		if path.isdir("_temp"): rmtree("_temp")
		
        mkdir("_temp")
		mkdir(path.join("_temp", "assets"))
		open(path.join("_temp", "assets", "db.db"), mode="a").close()
		
		copytree("web", 	 path.join("_temp", "static"))
		# copytree(path.join("assets", "snippets"), path.join("_temp", "snippets"))

		for f in [
			path.join("target", "release", "rusty_blog"),
		]:
			copy(f,"_temp")
		
		for f in [
			path.join("_temp", "static", "main.sass"),
			path.join("_temp", "static", "main.css.map"),
		]:
			if path.isfile(f): remove(f)

		make_archive("rusty_blog", "zip", "_temp")
		# rmtree("_temp")
    
    else:
        print("Nothing specified, aborting")


def compile_everything(release: bool, run_after: bool):	
	# compile misc
	run("sass-rs --sass --compressed < src/main.sass > static/main.min.css")
	run("minifier src/main.js")

    move("src/main.min.js", "static/main.min.js")

	# run main stuff
	if run_after:
		if release:
			run("cargo run --release")
		else:
			run("cargo run")
	else:
		if release:
			run("cargo build --release")
		else:
			run("cargo build")
    
    

if __name__ = "__main__":
    from sys import argv
    main(argv)
