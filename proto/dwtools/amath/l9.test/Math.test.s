( function _Math_test_s_() {

'use strict';

if( typeof module !== 'undefined' )
{

  let _ = require( '../../Tools.s' );

  _.include( 'wTesting' );

  require( '../../amath/l9/Include.s' );
  
}

var _global = _global_;
var _ = _global_.wTools;

// --
// tests
// --

function routinesOfMathScalar( test ) 
{
  test.case = 'namespace _.math';
  test.is( _.routineIs( _.math.fract ) );
  test.is( _.routineIs( _.math.factorial ) );
  test.is( _.routineIs( _.math.degToRad ) );
  test.is( _.routineIs( _.math.radToDeg ) );
  test.is( _.routineIs( _.math.clamp ) );
  test.is( _.routineIs( _.math.sqrt ) );
  test.is( _.routineIs( _.math.sqr ) );
  test.is( _.routineIs( _.math.cbd ) );
  test.is( _.routineIs( _.math.mod ) );
  test.is( _.routineIs( _.math.sign ) );
  test.is( _.routineIs( _.math.sc ) );
  test.is( _.routineIs( _.math.roundToPowerOfTwo ) );
  test.is( _.routineIs( _.math.ceilToPowerOfTwo ) );
  test.is( _.routineIs( _.math.floorToPowerOfTwo ) );
}

//

function routinesOfMathVector( test ) 
{
  test.case = 'namespace _.avector';
  test.is( _.routineIs( _.avector.add ) );
  test.is( _.routineIs( _.avector.sub ) );
  test.is( _.routineIs( _.avector.mul ) );
  test.is( _.routineIs( _.avector.div ) );
  test.is( _.routineIs( _.avector.min ) );
  test.is( _.routineIs( _.avector.max ) );

  test.case = 'namespace _.vectorAdapter';
  test.is( _.routineIs( _.vectorAdapter.add ) );
  test.is( _.routineIs( _.vectorAdapter.sub ) );
  test.is( _.routineIs( _.vectorAdapter.mul ) );
  test.is( _.routineIs( _.vectorAdapter.div ) );
  test.is( _.routineIs( _.vectorAdapter.min ) );
  test.is( _.routineIs( _.vectorAdapter.max ) );
}

//

function routinesOfMathMatrix( test ) 
{
  test.case = 'namespace _.Matrix';
  test.is( _.routineIs( _.Matrix.make ) );
  test.is( _.routineIs( _.Matrix.makeZero ) );
  test.is( _.routineIs( _.Matrix.makeSquare ) );
  test.is( _.routineIs( _.Matrix.makeCol ) );
  test.is( _.routineIs( _.Matrix.makeColZeroed ) );
  test.is( _.routineIs( _.Matrix.makeLine ) );
  test.is( _.routineIs( _.Matrix.makeRow ) );
  test.is( _.routineIs( _.Matrix.makeRowZeroed ) );
  test.is( _.routineIs( _.Matrix.makeSimilar ) );
  test.is( _.routineIs( _.Matrix.from ) );
}

//

function routinesOfMathModels( test )
{
  test.case = 'namespace _.box';
  test.is( _.routineIs( _.box.make ) );
  test.is( _.routineIs( _.box.makeZero ) );
  test.is( _.routineIs( _.box.makeNil ) );
  test.is( _.routineIs( _.box.zero ) );
  test.is( _.routineIs( _.box.nil ) );
  test.is( _.routineIs( _.box.centeredOfSize ) );
  test.is( _.routineIs( _.box.from ) );
  test.is( _.routineIs( _.box.adapterFrom ) );
  test.is( _.routineIs( _.box.fromPoints ) );
  test.is( _.routineIs( _.box.fromCenterAndSize ) );
  test.is( _.routineIs( _.box.fromSphere ) );
  test.is( _.routineIs( _.box.fromCube ) );
  
  test.case = 'namespace _.sphere';
  test.is( _.routineIs( _.sphere.make ) );
  test.is( _.routineIs( _.sphere.makeZero ) );
  test.is( _.routineIs( _.sphere.makeNil ) );
  test.is( _.routineIs( _.sphere.zero ) );
  test.is( _.routineIs( _.sphere.nil ) );
  test.is( _.routineIs( _.sphere.centeredOfRadius ) );
  test.is( _.routineIs( _.sphere.from ) );
  test.is( _.routineIs( _.sphere.adapterFrom ) );
  test.is( _.routineIs( _.sphere.fromPoints ) );
  test.is( _.routineIs( _.sphere.fromBox ) );
  test.is( _.routineIs( _.sphere.fromCenterAndRadius ) );
}

// --
// declare
// --

var Self =
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

Self = wTestSuite( Self );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
