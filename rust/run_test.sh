#!/bin/bash

# 

# cd into the folders in the current directory and run the test.py file
for folder in */
do
    cd "$folder"
    cargo test
    cd ..
done

