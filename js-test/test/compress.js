const assert = require('assert');
const lz_string_js = require('lz-string-js');
const lz_string_rs = require('lz-string'); // Can't alias this since it's not in registry...

const TEST_PHRASE = 'During tattooing, ink is injected into the skin, initiating an immune response, and cells called "macrophages" move into the area and "eat up" the ink. The macrophages carry some of the ink to the body\'s lymph nodes, but some that are filled with ink stay put, embedded in the skin. That\'s what makes the tattoo visible under the skin. Dalhousie Uiversity\'s Alec Falkenham is developing a topical cream that works by targeting the macrophages that have remained at the site of the tattoo. New macrophages move in to consume the previously pigment-filled macrophages and then migrate to the lymph nodes, eventually taking all the dye with them. "When comparing it to laser-based tattoo removal, in which you see the burns, the scarring, the blisters, in this case, we\'ve designed a drug that doesn\'t really have much off-target effect," he said. "We\'re not targeting any of the normal skin cells, so you won\'t see a lot of inflammation. In fact, based on the process that we\'re actually using, we don\'t think there will be any inflammation at all and it would actually be anti-inflammatory.';
const COMPRESSED_TEST_PHRASE = "ࢅ灎ॠ瘎怄Űↂၻ㝆ƣ됍枀칹䀕肦̘⅀⚧ꢀᘕ쑾틭Ј䊖㡈ꇠ୪␔㙠⤐æ訑હ萲ꅀඦ鉔鍯꜀ᄨꑔ솣鳉ౙ䛢譀෍됦ख़ঔ衔ن⡐㵜ꇰè唽䷍ⶭ浴郀쀁㷘텄\ud8d0Ì堣ঢ়㰀賐ǈ䤵㍄고ꂫ敱쩀ၲ菒\ud80a‌᠁\udd79饉ࢉ醲瀩䗋ඡ诙㢠揢僪궻䷱攷酐ム鰠蠡쬴\ud924훁㘹ꁣ臵飐䇮\ud800唈஌཰䋔䠀䄧锎\u0003៑鲠홑ㄎڱ物ⲂἺแϒ棠騣쐕ᦠ샸ሹ嬬茂\ud8f8끍ꉂ액뎜⧝欋踃┳䄌䅰묗辒⦋⻨㐌䀇ⅅᠸ찜钹퇏죙エ訢ࠗꘫ郉湿∃一䦁ࠀⴐ쑢ꩊ獒븆㡄䨠胀쀨㘙䶀퉪䵚欢⋅쭩Ϩⷈ涼ᘹ뚩뇆Ṣꢆ①Ϊ끄몦멐䯄䆠␲谇漪圌㋫ꂹ쾦躄즦修삠ت⸌呃큤끃誦軫䌆뙘眒嵷ਰꃕ李齔膃聀䘭ଢ胕뫞洶侖삑儦蔂鶱鳋聐ਅ樂ᬏ擱႐်ଶ쵗ᤅꍁ멧슁攓⠛␩㙖谅ㆉྟ㋐瑜ࢃⲻတ켞춁溞຀ਊ\udc40ᄔᢀґည㌋步Ƅ儶捍Ð蠒ჱ屪倩覍⽠黥膗㘎蟂⃦⥀䅢裘㒀ꊐ⨯胣贈ⵃ솺⩄ӑ뜚ш둖式莵죊㓇䂲⢈\u0000";

describe('Compress', function () {
    describe('#compress', function () {
        it('should have both js and rs compress the same', function () {
            let compressed_js = lz_string_js.compress(TEST_PHRASE);
            let compressed_rs = lz_string_rs.compress(TEST_PHRASE);
            assert.equal(compressed_js, compressed_rs);
            assert.equal(compressed_js, COMPRESSED_TEST_PHRASE);
            assert.equal(compressed_rs, COMPRESSED_TEST_PHRASE);
        });
    });
});

describe('Decompress', function () {
    describe('#decompress', function () {
        it('should have both js and rs decompress the same', function () {
            let decompressed_js = lz_string_js.decompress(COMPRESSED_TEST_PHRASE);
            let decompressed_rs = lz_string_rs.decompress(COMPRESSED_TEST_PHRASE);
            assert.equal(decompressed_js, decompressed_rs);
            assert.equal(decompressed_js, TEST_PHRASE);
            assert.equal(decompressed_rs, TEST_PHRASE);
        });
    });
});