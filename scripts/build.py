#!/usr/bin/python

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
		copytree("static", "_temp")
		run("cargo run --release --bin build_static")

		make_archive("static", "zip", "_temp")
        # rmtree("_temp")
    
    # exporting the whole thing to a zip file
	elif "--export" in args:
		compile_everything(True, False)
		if path.isdir("_temp"): rmtree("_temp")
		
		mkdir("_temp")
		copytree("assets", path.join("_temp", "assets"))
		copytree("static", path.join("_temp", "static"))

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
		compile_everything(False, True)


def compile_everything(release: bool, run_after: bool):	
	# compile misc
	run("sass-rs --sass --compressed < src/main.sass > static/main.min.css")
	run("minifier src/main.js")
	run("minifier src/article_edit.js")
	

	move("src/main.min.js", "static/main.min.js")
	move("src/article_edit.min.js", "static/article_edit.min.js")


	# run main stuff
	run_cmd = "cargo " + ("run" if run_after else "build") + (" --release" if release else "")
	run(run_cmd)


if __name__ == "__main__":
	from sys import argv
	main(argv)
