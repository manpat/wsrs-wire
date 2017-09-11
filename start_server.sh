#!/usr/bin/env bash

cd server > /dev/null

for session in $(screen -ls | grep -o '[0-9]*\.wire_server'); do
	screen -S "${session}" -X quit;
done

screen -dmS "wire_server" bash -i
sleep 0.1 # shutup
screen -r -S "wire_server" -p 0 -X stuff $'RUST_BACKTRACE=1 cargo run\r'
