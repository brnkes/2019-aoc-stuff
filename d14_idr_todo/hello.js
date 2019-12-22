#!/usr/bin/env node
"use strict";

(function(){

const $JSRTS = {
    throw: function (x) {
        throw x;
    },
    Lazy: function (e) {
        this.js_idris_lazy_calc = e;
        this.js_idris_lazy_val = void 0;
    },
    force: function (x) {
        if (x === undefined || x.js_idris_lazy_calc === undefined) {
            return x
        } else {
            if (x.js_idris_lazy_val === undefined) {
                x.js_idris_lazy_val = x.js_idris_lazy_calc()
            }
            return x.js_idris_lazy_val
        }
    },
    prim_strSubstr: function (offset, len, str) {
        return str.substr(Math.max(0, offset), Math.max(0, len))
    }
};
$JSRTS.os = require('os');

$JSRTS.fs = require('fs');

$JSRTS.prim_systemInfo = function (index) {
    switch (index) {
        case 0:
            return "node";
        case 1:
            return $JSRTS.os.platform();
    }
    return "";
};

$JSRTS.prim_writeStr = function (x) { return process.stdout.write(x) };

$JSRTS.prim_readStr = function () {
    var ret = '';
    var b = Buffer.alloc(1024);
    var i = 0;
    while (true) {
        $JSRTS.fs.readSync(0, b, i, 1)
        if (b[i] == 10) {
            ret = b.toString('utf8', 0, i);
            break;
        }
        i++;
        if (i == b.length) {
            var nb = Buffer.alloc(b.length * 2);
            b.copy(nb)
            b = nb;
        }
    }
    return ret;
};

$JSRTS.die = function (message) {
    console.error(message);
    process.exit(-1);
};





const $HC_0_0$MkUnit = ({type: 0});
const $HC_0_0$TheWorld = ({type: 0});
// Main.main

function Main__main($_0_in){
    const $_1_in = "10 A => 1 FUEL";
    return $HC_0_0$MkUnit;
}

// {runMain_0}

function $_0_runMain(){
    return $JSRTS.force(Main__main($HC_0_0$TheWorld));
}


$_0_runMain();
}.call(this))