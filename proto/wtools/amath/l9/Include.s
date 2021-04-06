//#! /usr/bin/env node
( function _Include_s_()
{

'use strict';

if( typeof module !== 'undefined' && module !== null )
{
  const _ = require( 'Tools' );

  _.include( 'wMathScalar' );
  _.include( 'wMathVector' );
  _.include( 'wMathMatrix' );
  _.include( 'wMathModels' );
}

const _global = _global_;
const _ = _global_.wTools;
const Self = _global_.wTools;

// --
// export
// --

if( typeof module !== 'undefined' && module !== null )
module[ 'exports' ] = Self;

})();
