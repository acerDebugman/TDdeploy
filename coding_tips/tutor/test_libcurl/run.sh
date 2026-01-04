#!/bin/bash


# apt-get install libcurl4-openssl-dev

gcc -o c_client curl_client.c -lcurl -std=c99 -Wall

