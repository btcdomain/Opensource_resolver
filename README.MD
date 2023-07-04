i64 ----> BigInt
i32 ----> Integer

setsid ./ord --data-dir=/home/ubuntu/.local/share2/ord server --http-port 8092 >> ord_http2.log 2>&1 &

setsid ./ord server --http-port 8091 >> ord_http.log 2>&1 &