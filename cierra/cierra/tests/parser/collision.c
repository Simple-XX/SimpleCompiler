//@ ensures \true;
void collide() {
    int x = 0;
    int y = 0;
    //@ loop invariant x <= 5;
    while (x < 5) {
        int y = x;
        x = y + 1;
    }
    //@ assert y == 0;
    //@ assert x == 5;
}