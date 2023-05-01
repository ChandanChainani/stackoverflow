var should = require('should');

var array = [
 'a',   'b',
 'c',   'd',
 'e',   'f',
 'g',   'h',
 'i',   'j'
];

array.should.be.a.Array()
  .and.deepEqual(array)
.with.lengthOf(array.length);
