#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: $0 <day_number>"
    exit 1
fi

DAY_NUMBER=$1
DAY_NAME="day_$DAY_NUMBER"
DAY_FILE="src/days/$DAY_NAME.rs"
DAY_INPUT="input/day$DAY_NUMBER.txt"

if [ -f "$DAY_INPUT" ]; then
    echo "File $DAY_INPUT already exists"
    exit 1
fi

touch "$DAY_INPUT"

if [ -f "$DAY_FILE" ]; then
    echo "File $DAY_FILE already exists"
    exit 1
fi

cat > "$DAY_FILE" <<EOF
use super::DaySolver;

pub struct Solver;

impl DaySolver for Solver {
    fn q2(&self, input_txt: &str) -> String {
        todo!()
    }

    fn q1(&self, input_txt: &str) -> String {
        todo!()
    }
}
EOF
# end of script