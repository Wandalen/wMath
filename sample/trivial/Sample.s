if( typeof module !== undefined )
var _ = require( 'wmath' );

/* wMathScalar sample */
console.log( '= Scalar' );
var fract = _.math.fract( 1.1 );
console.log( 'Fract', fract, '\n' );

/* wMathVector sample */

console.log( '= Vector' );
var a1 = [ 1, 2, 5, 9 ];
var a2 = [ 1, 2, 3, 4 ];
_.avector.add( a1, a2 );
console.log( 'add 2 avectors' );
console.log( 'a1', a1 );
console.log( 'a2', a2, '\n' );

/* wMathMatrix sample */

console.log( '= Matrix' );
var u = _.Matrix.Make([ 3, 3 ]).copy
([
  +1, +2, +3,
  +0, +4, +5,
  +0, +0, +6,
]);

var l = _.Matrix.Make([ 3, 3 ]).copy
([
  +1, +0, +0,
  +2, +4, +0,
  +3, +5, +6,
]);

var expected = _.Matrix.Make([ 3, 3 ]).copy
([
  +14, +23, +18,
  +23, +41, +30,
  +18, +30, +36,
]);

var uxl = _.Matrix.Mul( null, [ u, l ] );
console.log( 'got\n' + uxl.toStr() );
console.log( 'expected\n' + expected.toStr(), '\n' );

/* wMathModels sample */

console.log( '= Concepts' );

var got = _.box.make();

console.log( '3D Box created', got );

