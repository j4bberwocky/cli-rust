#!/usr/bin/env bash

OUTDIR="expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

echo "Hello girls" > $OUTDIR/hello1.txt
echo "Hello" "girls" > $OUTDIR/hello2.txt
echo -n "Hello girls" > $OUTDIR/hello1.n.txt
echo -n "Hello" "girls" > $OUTDIR/hello2.n.txt