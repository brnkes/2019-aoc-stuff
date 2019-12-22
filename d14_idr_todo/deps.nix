
{
  build-idris-package
, idrisscript
, lib
}:
build-idris-package  {
    name = "aoc209-day14";
    version = "0.0.1";
    ipkgName = "d14";
    idrisDeps = [ idrisscript ];

    src = ./.;

    meta = {
        license = lib.licenses.mit;
    };
}