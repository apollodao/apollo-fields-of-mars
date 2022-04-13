json2ts -i ../contracts/martian-field/schema/ -o ./


for f in ./*.d.ts; do
    mv -- "$f" "${f%.d.ts}.ts"
done