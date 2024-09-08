#include <stdio.h>
#include <fcntl.h>
#include <io.h>

void setfilebinary() {
    _setmode(_fileno(stdin), _O_BINARY);
    _setmode(_fileno(stdout), _O_BINARY);
}