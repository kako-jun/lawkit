# International Numeral Systems Research

## Current Support Status
- ✅ **Japanese**: Full-width digits (０１２３), Kanji numerals (一二三), Positional notation (一千二百)
- 🟡 **Chinese**: Basic kanji (一二三) work, but missing financial forms and traditional variants
- ❌ **Other systems**: Not supported

## Major Numeral Systems to Consider

### 1. Chinese Numerals

#### Basic Forms (Already supported via Japanese)
- 零一二三四五六七八九 (0-9)
- 十百千万 (10, 100, 1000, 10000)

#### Traditional/Financial Forms (NOT supported)
- **Financial digits**: 壹貳參肆伍陸柒捌玖拾
- **Traditional 万**: 萬 (vs Japanese 万)
- **Higher units**: 億兆京垓 (100M, 1T, 10^16, 10^20)

#### Regional Variants
- **Traditional Chinese**: 萬億兆 (Taiwan, Hong Kong)
- **Simplified Chinese**: 万亿兆 (Mainland China)

### 2. Arabic-Indic Numerals

#### Eastern Arabic Numerals
- ٠١٢٣٤٥٦٧٨٩ (0-9)
- Used in: Middle East, Arabic countries

#### Persian/Farsi Numerals  
- ۰۱۲۳۴۵۶۷۸۹ (0-9)
- Used in: Iran, Afghanistan

### 3. South Asian Numerals

#### Devanagari (Hindi)
- ०१२३४५६७८९ (0-9)
- Used in: India (Hindi), Nepal

#### Bengali
- ০১২৩৪৫৬৭৮৯ (0-9)
- Used in: Bangladesh, West Bengal

#### Gujarati  
- ૦૧૨૩૪૫૬૭૮૯ (0-9)
- Used in: Gujarat state, India

#### Tamil
- ௦௧௨௩௪௫௬௭௮௯ (0-9)
- Used in: Tamil Nadu, Sri Lanka

### 4. Southeast Asian Numerals

#### Thai
- ๐๑๒๓๔๕๖๗๘๙ (0-9)
- Used in: Thailand

#### Myanmar
- ၀၁၂၃၄၅၆၇၈၉ (0-9)
- Used in: Myanmar

#### Khmer
- ០១២៣៤៥៦៧៨៩ (0-9)
- Used in: Cambodia

### 5. Other Systems

#### Tibetan
- ༠༡༢༣༤༥༦༧༨༩ (0-9)
- Used in: Tibet, Bhutan

#### Mongolian
- ᠐᠑᠒᠓᠔᠕᠖᠗᠘᠙ (0-9)
- Used in: Mongolia (traditional script)

#### Ethiopian
- ፩፪፫፬፭፮፯፰፱ (1-9), no zero
- Used in: Ethiopia

## Implementation Priority

### High Priority
1. **Chinese Financial Numerals**: 壹貳參肆伍陸柒捌玖
2. **Arabic-Indic**: ٠١٢٣٤٥٦٧٨٩ (widely used)
3. **Persian**: ۰۱۲۳۴۵۶۷۸۹ (significant user base)

### Medium Priority  
4. **Hindi/Devanagari**: ०१२३४५६७८९
5. **Thai**: ๐๑๒๓๔๕๖๗๘๙
6. **Bengali**: ০১২৩৪৫৬৭৮৯

### Lower Priority
7. Other regional scripts (case-by-case basis)

## Technical Considerations

### Unicode Support
- All mentioned numerals have Unicode code points
- Regex patterns need to be expanded
- Full-width vs half-width variants exist for some

### Cultural Context
- **Financial forms** prevent fraud (Chinese 壹 vs 一)
- **Regional preferences** (萬 vs 万)
- **Legacy systems** may use old variants

### User Names Edge Case
- Chinese names with number characters (一郎, 二子) are rare in fraud contexts
- Japanese names with kanji numbers (一, 二, 三) as name components should be excluded from analysis when in name context
- Solution: Context-aware processing or explicit number-only mode

## Recommended Implementation Strategy

### Phase 1: Chinese Financial Support
Add support for Chinese traditional financial numerals to prevent fraud detection in Chinese financial documents.

### Phase 2: Major Arabic Scripts  
Add Arabic-Indic and Persian numerals for Middle Eastern document analysis.

### Phase 3: South Asian Scripts
Add Hindi/Devanagari support for Indian market.

### Phase 4: Additional Scripts
Add other scripts based on user demand and usage patterns.