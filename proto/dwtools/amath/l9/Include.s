//#! /usr/bin/env node
( function _Include_s_(){

'use strict';

if( typeof module !== 'undefined' && module !== null )
{
  let _ = require( '../../Tools.s' );

  _.include( 'wMathScalar' );
  _.include( 'wMathVector' );
  _.include( 'wMathMatrix' );
  _.include( 'wMathConcepts' );
}

let _global = _global_;
let _ = _global_.wTools;
let Self = _global_.wTools;

// --
// export
// --

if( typeof module !== 'undefined' && module !== null )
module[ 'exports' ] = Self;

})();
