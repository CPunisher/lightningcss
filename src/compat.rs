// This file is autogenerated by build-prefixes.js. DO NOT EDIT!

use crate::targets::Browsers;

pub enum Feature {
  Clamp,
  CssAnyLink,
  CssAutofill,
  CssCaseInsensitive,
  CssDefaultPseudo,
  CssDirPseudo,
  CssFirstLetter,
  CssFirstLine,
  CssFocusVisible,
  CssFocusWithin,
  CssGencontent,
  CssInOutOfRange,
  CssIndeterminatePseudo,
  CssMarkerPseudo,
  CssMatchesPseudo,
  CssNamespaces,
  CssNesting,
  CssOptionalPseudo,
  CssPlaceholder,
  CssPlaceholderShown,
  CssReadOnlyWrite,
  CssRrggbbaa,
  CssSel2,
  CssSel3,
  CssSelection,
  Dialog,
  DoublePositionGradients,
  FormValidation,
  Fullscreen,
  LogicalBorderRadius,
  LogicalBorders,
  MediaIntervalSyntax,
  MediaRangeSyntax,
  OverflowShorthand,
  PlaceContent,
  PlaceItems,
  PlaceSelf,
  Shadowdomv1
}

impl Feature {
  pub fn is_compatible(&self, browsers: Browsers) -> bool {
    match self {
      Feature::CssSel2 => {
        if let Some(version) = browsers.ie {
          if version >= 458752 {
            return true
          }
        }
        if let Some(version) = browsers.edge {
          if version >= 786432 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 131072 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 262144 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 196864 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 589824 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 197120 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 131328 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 262144 {
            return true
          }
        }
      }
      Feature::CssSel3 => {
        if let Some(version) = browsers.ie {
          if version >= 589824 {
            return true
          }
        }
        if let Some(version) = browsers.edge {
          if version >= 786432 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 197888 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 262144 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 197120 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 591104 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 197120 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 131328 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 262144 {
            return true
          }
        }
      }
      Feature::CssGencontent |
      Feature::CssFirstLine => {
        if let Some(version) = browsers.ie {
          if version >= 589824 {
            return true
          }
        }
        if let Some(version) = browsers.edge {
          if version >= 786432 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 131072 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 262144 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 196864 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 589824 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 197120 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 131328 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 262144 {
            return true
          }
        }
      }
      Feature::CssFirstLetter => {
        if let Some(version) = browsers.ie {
          if version >= 589824 {
            return true
          }
        }
        if let Some(version) = browsers.edge {
          if version >= 786432 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 197888 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 589824 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 327936 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 722432 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 327680 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 196608 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 262144 {
            return true
          }
        }
      }
      Feature::CssInOutOfRange => {
        if let Some(version) = browsers.edge {
          if version >= 5177344 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 3276800 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 3473408 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 655616 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 2621440 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 656128 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 6160384 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 327680 {
            return true
          }
        }
      }
      Feature::FormValidation => {
        if let Some(version) = browsers.ie {
          if version >= 655360 {
            return true
          }
        }
        if let Some(version) = browsers.edge {
          if version >= 786432 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 262144 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 655360 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 655616 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 655360 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 656128 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 263171 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 262144 {
            return true
          }
        }
      }
      Feature::CssAnyLink => {
        if let Some(version) = browsers.edge {
          if version >= 5177344 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 3276800 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 4259840 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 589824 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 3407872 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 589824 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 6160384 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 590336 {
            return true
          }
        }
      }
      Feature::CssDefaultPseudo => {
        if let Some(version) = browsers.edge {
          if version >= 5177344 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 262144 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 3342336 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 655616 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 2490368 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 656128 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 6160384 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 327680 {
            return true
          }
        }
      }
      Feature::CssDirPseudo => {
        if let Some(version) = browsers.firefox {
          if version >= 3211264 {
            return true
          }
        }
      }
      Feature::CssFocusWithin => {
        if let Some(version) = browsers.edge {
          if version >= 5177344 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 3407872 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 3932160 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 655616 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 3080192 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 656128 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 6160384 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 524800 {
            return true
          }
        }
      }
      Feature::CssFocusVisible => {
        if let Some(version) = browsers.edge {
          if version >= 5636096 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 5570560 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 5636096 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 4718592 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 6160384 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 917504 {
            return true
          }
        }
      }
      Feature::CssIndeterminatePseudo => {
        if let Some(version) = browsers.edge {
          if version >= 5177344 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 3342336 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 2555904 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 655616 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 1703936 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 656128 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 6160384 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 262144 {
            return true
          }
        }
      }
      Feature::CssMatchesPseudo => {
        if let Some(version) = browsers.edge {
          if version >= 5767168 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 5111808 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 5767168 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 917504 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 4915200 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 917504 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 6160384 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 983040 {
            return true
          }
        }
      }
      Feature::CssOptionalPseudo => {
        if let Some(version) = browsers.ie {
          if version >= 655360 {
            return true
          }
        }
        if let Some(version) = browsers.edge {
          if version >= 786432 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 262144 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 983040 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 327680 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 983040 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 327680 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 131840 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 262144 {
            return true
          }
        }
      }
      Feature::CssPlaceholderShown => {
        if let Some(version) = browsers.edge {
          if version >= 5177344 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 3342336 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 3080192 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 589824 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 2228224 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 589824 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 6160384 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 327680 {
            return true
          }
        }
      }
      Feature::Dialog => {
        if let Some(version) = browsers.edge {
          if version >= 5177344 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 2424832 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 1572864 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 6160384 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 262144 {
            return true
          }
        }
      }
      Feature::Fullscreen => {
        if let Some(version) = browsers.edge {
          if version >= 5177344 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 4194304 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 4653056 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 786688 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 655616 {
            return true
          }
        }
      }
      Feature::CssMarkerPseudo => {
        if let Some(version) = browsers.edge {
          if version >= 5636096 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 4456448 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 5636096 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 4718592 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 721664 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 6160384 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 917504 {
            return true
          }
        }
      }
      Feature::CssPlaceholder => {
        if let Some(version) = browsers.edge {
          if version >= 5177344 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 3342336 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 3735552 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 655616 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 2883584 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 656128 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 6160384 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 459264 {
            return true
          }
        }
      }
      Feature::CssSelection => {
        if let Some(version) = browsers.ie {
          if version >= 589824 {
            return true
          }
        }
        if let Some(version) = browsers.edge {
          if version >= 786432 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 4063232 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 262144 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 196864 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 591104 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 263168 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 262144 {
            return true
          }
        }
      }
      Feature::CssCaseInsensitive => {
        if let Some(version) = browsers.edge {
          if version >= 5177344 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 3080192 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 3211264 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 589824 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 2359296 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 589824 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 6160384 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 327680 {
            return true
          }
        }
      }
      Feature::CssReadOnlyWrite => {
        if let Some(version) = browsers.edge {
          if version >= 851968 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 5111808 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 2359296 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 589824 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 1507328 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 589824 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 6160384 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 262144 {
            return true
          }
        }
      }
      Feature::CssAutofill => {
        if let Some(version) = browsers.chrome {
          if version >= 6291456 {
            return true
          }
        }
        if let Some(version) = browsers.edge {
          if version >= 6291456 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 5636096 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 5373952 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 6291456 {
            return true
          }
        }
      }
      Feature::CssNamespaces => {
        if let Some(version) = browsers.ie {
          if version >= 589824 {
            return true
          }
        }
        if let Some(version) = browsers.edge {
          if version >= 786432 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 131072 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 262144 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 262144 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 589824 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 262656 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 131328 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 262144 {
            return true
          }
        }
      }
      Feature::Shadowdomv1 => {
        if let Some(version) = browsers.edge {
          if version >= 5177344 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 4128768 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 3473408 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 655360 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 2621440 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 720896 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 6160384 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 393728 {
            return true
          }
        }
      }
      Feature::CssRrggbbaa => {
        if let Some(version) = browsers.edge {
          if version >= 5177344 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 3211264 {
            return true
          }
        }
        if let Some(version) = browsers.chrome {
          if version >= 4063232 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 655360 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 3407872 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 655360 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 6160384 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 524800 {
            return true
          }
        }
      }
      Feature::CssNesting |
      Feature::MediaIntervalSyntax => {}
      Feature::DoublePositionGradients => {
        if let Some(version) = browsers.chrome {
          if version >= 4653056 {
            return true
          }
        }
        if let Some(version) = browsers.edge {
          if version >= 5177344 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 4194304 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 3276800 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 786688 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 786944 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 655360 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 4653056 {
            return true
          }
        }
      }
      Feature::Clamp => {
        if let Some(version) = browsers.chrome {
          if version >= 5177344 {
            return true
          }
        }
        if let Some(version) = browsers.edge {
          if version >= 5177344 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 5177344 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 3735552 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 852224 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 852992 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 786432 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 5177344 {
            return true
          }
        }
      }
      Feature::PlaceSelf |
      Feature::PlaceItems => {
        if let Some(version) = browsers.chrome {
          if version >= 3866624 {
            return true
          }
        }
        if let Some(version) = browsers.edge {
          if version >= 5177344 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 2949120 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 2818048 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 720896 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 720896 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 458752 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 3866624 {
            return true
          }
        }
      }
      Feature::PlaceContent => {
        if let Some(version) = browsers.chrome {
          if version >= 3866624 {
            return true
          }
        }
        if let Some(version) = browsers.edge {
          if version >= 5177344 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 2949120 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 2818048 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 589824 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 589824 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 458752 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 3866624 {
            return true
          }
        }
      }
      Feature::OverflowShorthand => {
        if let Some(version) = browsers.chrome {
          if version >= 4456448 {
            return true
          }
        }
        if let Some(version) = browsers.edge {
          if version >= 5177344 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 3997696 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 3145728 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 655360 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 4456448 {
            return true
          }
        }
      }
      Feature::MediaRangeSyntax => {
        if let Some(version) = browsers.firefox {
          if version >= 4128768 {
            return true
          }
        }
      }
      Feature::LogicalBorders => {
        if let Some(version) = browsers.chrome {
          if version >= 4521984 {
            return true
          }
        }
        if let Some(version) = browsers.edge {
          if version >= 5177344 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 2686976 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 3145728 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 786688 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 786944 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 655360 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 4521984 {
            return true
          }
        }
      }
      Feature::LogicalBorderRadius => {
        if let Some(version) = browsers.chrome {
          if version >= 5832704 {
            return true
          }
        }
        if let Some(version) = browsers.edge {
          if version >= 5832704 {
            return true
          }
        }
        if let Some(version) = browsers.firefox {
          if version >= 4325376 {
            return true
          }
        }
        if let Some(version) = browsers.opera {
          if version >= 4915200 {
            return true
          }
        }
        if let Some(version) = browsers.safari {
          if version >= 983040 {
            return true
          }
        }
        if let Some(version) = browsers.ios_saf {
          if version >= 983040 {
            return true
          }
        }
        if let Some(version) = browsers.samsung {
          if version >= 983040 {
            return true
          }
        }
        if let Some(version) = browsers.android {
          if version >= 5832704 {
            return true
          }
        }
      }
    }
    false
  }
}
