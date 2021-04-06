( function _Math_test_s_()
{

'use strict';

if( typeof module !== 'undefined' )
{

  const _ = require( 'Tools' );

  _.include( 'wTesting' );

  require( '../../amath/l9/Include.s' );

}

const _global = _global_;
const _ = _global_.wTools;

// --
// tests
// --

function routinesOfMathScalar( test )
{
  test.case = 'namespace _.math';
  test.true( _.routineIs( _.math.fract ) );
  test.true( _.routineIs( _.math.factorial ) );
  test.true( _.routineIs( _.math.degToRad ) );
  test.true( _.routineIs( _.math.radToDeg ) );
  test.true( _.routineIs( _.math.clamp ) );
  test.true( _.routineIs( _.math.sqrt ) );
  test.true( _.routineIs( _.math.sqr ) );
  test.true( _.routineIs( _.math.cbd ) );
  test.true( _.routineIs( _.math.mod ) );
  test.true( _.routineIs( _.math.sign ) );
  test.true( _.routineIs( _.math.sc ) );
  test.true( _.routineIs( _.math.roundToPowerOfTwo ) );
  test.true( _.routineIs( _.math.ceilToPowerOfTwo ) );
  test.true( _.routineIs( _.math.floorToPowerOfTwo ) );
}

//

function routinesOfMathVector( test )
{
  test.case = 'namespace _.avector';
  test.true( _.routineIs( _.avector.add ) );
  test.true( _.routineIs( _.avector.sub ) );
  test.true( _.routineIs( _.avector.mul ) );
  test.true( _.routineIs( _.avector.div ) );
  test.true( _.routineIs( _.avector.min ) );
  test.true( _.routineIs( _.avector.max ) );

  test.case = 'namespace _.vectorAdapter';
  test.true( _.routineIs( _.vectorAdapter.add ) );
  test.true( _.routineIs( _.vectorAdapter.sub ) );
  test.true( _.routineIs( _.vectorAdapter.mul ) );
  test.true( _.routineIs( _.vectorAdapter.div ) );
  test.true( _.routineIs( _.vectorAdapter.min ) );
  test.true( _.routineIs( _.vectorAdapter.max ) );
}

//

function routinesOfMathMatrix( test )
{
  test.case = 'namespace _.Matrix';
  test.true( _.routineIs( _.Matrix.Make ) );
  test.true( _.routineIs( _.Matrix.MakeZero ) );
  test.true( _.routineIs( _.Matrix.MakeSquare ) );
  test.true( _.routineIs( _.Matrix.MakeCol ) );
  test.true( _.routineIs( _.Matrix.MakeColZeroed ) );
  test.true( _.routineIs( _.Matrix.MakeLine ) );
  test.true( _.routineIs( _.Matrix.MakeRow ) );
  test.true( _.routineIs( _.Matrix.MakeRowZeroed ) );
  test.true( _.routineIs( _.Matrix.MakeSimilar ) );
  test.true( _.routineIs( _.Matrix.From ) );
}

//

function routinesOfMathModels( test )
{
  test.case = 'namespace _.box';
  test.true( _.routineIs( _.box.make ) );
  test.true( _.routineIs( _.box.makeZero ) );
  // test.true( _.routineIs( _.box.makeNil ) ); /* doesn't exist */
  test.true( _.routineIs( _.box.zero ) );
  test.true( _.routineIs( _.box.nil ) );
  test.true( _.routineIs( _.box.centeredOfSize ) );
  test.true( _.routineIs( _.box.from ) );
  test.true( _.routineIs( _.box.adapterFrom ) );
  test.true( _.routineIs( _.box.fromPoints ) );
  test.true( _.routineIs( _.box.fromCenterAndSize ) );
  test.true( _.routineIs( _.box.fromSphere ) );
  test.true( _.routineIs( _.box.fromCube ) );

  test.case = 'namespace _.sphere';
  test.true( _.routineIs( _.sphere.make ) );
  test.true( _.routineIs( _.sphere.makeZero ) );
  // test.true( _.routineIs( _.sphere.makeNil ) ); /* doesn't exist */
  test.true( _.routineIs( _.sphere.zero ) );
  test.true( _.routineIs( _.sphere.nil ) );
  test.true( _.routineIs( _.sphere.centeredOfRadius ) );
  test.true( _.routineIs( _.sphere.from ) );
  test.true( _.routineIs( _.sphere.adapterFrom ) );
  test.true( _.routineIs( _.sphere.fromPoints ) );
  test.true( _.routineIs( _.sphere.fromBox ) );
  test.true( _.routineIs( _.sphere.fromCenterAndRadius ) );
}

// --
// declare
// --

const Proto =
{

  name : 'Tools.math.l9.Math',
  silencing : 1,

  tests :
  {

    routinesOfMathScalar,
    routinesOfMathVector,
    routinesOfMathMatrix,
    routinesOfMathModels,

  },

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
