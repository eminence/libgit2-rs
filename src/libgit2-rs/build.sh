gcc --shared -o libachin.so test.c -fPIC && echo "Done with test.c" && \
~/devel/rust/prefix/bin/rustc lib.rs && echo "done with lib.rs" && \
~/devel/rust/prefix/bin/rustc -L. --link-args -lgit2 --test test.rs && echo "done with test.rs"
