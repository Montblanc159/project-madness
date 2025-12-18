#!/bin/sh

ink_files=`ls -1 scenario/dialogs/*.ink.json | xargs -n 1 basename`

file_diffs=""

for entry in $ink_files
do
    file_diffs=`${file_diffs} grep -xvFf scenario/dialogs/"$entry" dev/assets/dialogs/"$entry"`
done

if [ -z "$file_diffs" ]
then
    echo "Dialogs OK"
    exit 0
else
    echo "Diffs in dialogs:"

    for diff in $file_diffs
    do
        echo "${diff}"
    done

    exit 1
fi
