echo $(echo foo bar | awk '{print $NF}')
echo $(echo foo bar z | awk '{print $NF}')

echo `echo foo bar1 | awk '{print $NF}'`
echo `echo foo bar z1 | awk '{print $NF}'`

VER1=`echo foo bar baz | awk '{print $NF}'`
echo $VER1

VER2=$(echo foo bar baz1 | awk '{print $NF}')
echo $VER2
