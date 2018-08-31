class Solution {
public:
    // int findNthDigit(int n) {
    //     int width = 1, base = 1;
    //     while ((n - 1) / width + 1 > base * 9) {
    //         n -= width * base * 9;
    //         width++;
    //         base *= 10;
    //     }
    //     n--;
    //     int num = base + n / width;
    //     for (n = width - n % width - 1; n > 0; n--)
    //         num /= 10;
    //     return num % 10;
    // }

    int findNthDigit(int n) {
        int i;
        __asm__(R"==(
	movl	%1, %%ecx
	movl	$1, %%edi
	movl	$1, %%esi
.AAAA:
	movl	%%ecx, %%eax
	movl	$0, %%edx
	decl	%%eax
	idivl	%%edi
	incl	%%eax
	imull	$9, %%esi, %%ebx
	cmpl	%%ebx, %%eax
	jle	.AAAB

	imull	%%edi, %%ebx
	subl	%%ebx, %%ecx
	incl	%%edi
	imull	$10, %%esi, %%esi
	jmp	.AAAA
.AAAB:
	decl	%%ecx
	movl	%%ecx, %%eax
	movl	$0, %%edx
	idivl	%%edi
	addl	%%eax, %%esi
	subl	%%edx, %%edi
	decl	%%edi

.AAAC:
	cmpl	$0, %%edi
	jle	.AAAD
	movl	%%esi, %%eax
	movl	$0, %%edx
	movl	$10, %%ecx
	idivl	%%ecx
	movl	%%eax, %%esi
	decl	%%edi
	jmp	.AAAC
.AAAD:
	movl	%%esi, %%eax
	movl	$0, %%edx
	movl	$10, %%ecx
	idivl	%%ecx

	movl	%%edx, %0
)=="
            :"=r"(i)
            :"r"(n)
            :"%eax", "%ebx", "%ecx", "%edx", "%edi", "%esi"
	    );
        return i;
    }
};