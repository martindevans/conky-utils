vcgencmd measure_clock arm | cut -c 15- | xargs -I{} awk "BEGIN {printf \"%.2f\n\", {}/1000000}"
