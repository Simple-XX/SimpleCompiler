/*@ requires \valid(a+(0..len)) && len >= 2;
  @ ensures \result == a[0] + a[1] + a[2];
 */
int fun(int a[], int len) {
    return a[0] + a[1] + a[2];
}
