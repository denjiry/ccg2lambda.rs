#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

use serde_xml_rs::from_reader;

#[test]
fn test() {
    parse_jigg()
}

#[derive(Debug, Deserialize)]
struct Token {
    pub reading: String,
    pub base: String,
    #[serde(rename = "inflectionForm", default)]
    pub inflection_form: String,
    #[serde(rename = "inflectionType", default)]
    pub inflection_type: String,
    pub pos3: String,
    pub pos2: String,
    pub pos1: String,
    pub pos: String,
    pub surf: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
struct CCG {
    pub score: String,
    pub id: String,
    pub root: String,
}

#[derive(Debug, Deserialize)]
struct Tokens {
    pub token: Vec<Token>,
}

#[derive(Debug, Deserialize)]
struct Sentence {
    pub id: String,
    pub tokens: Tokens,
    pub ccg: Vec<CCG>,
}

#[derive(Debug, Deserialize)]
struct Sentences {
    pub sentence: Vec<Sentence>,
}

#[derive(Debug, Deserialize)]
struct Document {
    pub id: String,
    pub sentences: Sentences,
}

#[derive(Debug, Deserialize)]
struct Root {
    #[serde(rename = "document", default)]
    pub documents: Vec<Document>,
}

pub fn parse_jigg() {
    let s = r##"
<?xml version='1.0' encoding='UTF-8'?>
<root>
  <document id="d0">
    <sentences>
      <sentence id="s0">
        ソクラテスは人間である。
        <tokens>
          <token reading="ソクラテス" base="ソクラテス" inflectionForm="*" inflectionType="*" pos3="姓" pos2="人名" pos1="固有名詞" pos="名詞" surf="ソクラテス" id="s0_0"/>
          <token reading="ワ" base="は" inflectionForm="*" inflectionType="*" pos3="*" pos2="*" pos1="係助詞" pos="助詞" surf="は" id="s0_1"/>
          <token reading="ニンゲン" base="人間" inflectionForm="*" inflectionType="*" pos3="*" pos2="*" pos1="一般" pos="名詞" surf="人間" id="s0_2"/>
          <token reading="デ" base="だ" inflectionForm="連用形" inflectionType="特殊・ダ" pos3="*" pos2="*" pos1="*" pos="助動詞" surf="で" id="s0_3"/>
          <token reading="アル" base="ある" inflectionForm="基本形" inflectionType="五段・ラ行アル" pos3="*" pos2="*" pos1="*" pos="助動詞" surf="ある" id="s0_4"/>
          <token reading="。" base="。" inflectionForm="*" inflectionType="*" pos3="*" pos2="*" pos1="句点" pos="記号" surf="。" id="s0_5"/>
        </tokens>
        <ccg score="544.7356014847755" id="s0_ccg0" root="s0_sp0">
          <span child="s0_sp1 s0_sp10" rule="&lt;" category="S[mod=nm,form=base,fin=t]" end="6" begin="0" id="s0_sp0"/>
          <span child="s0_sp2 s0_sp5" rule="&gt;" category="S[mod=nm,form=base,fin=f]" end="5" begin="0" id="s0_sp1"/>
          <span child="s0_sp3 s0_sp4" rule="&lt;" category="S[fin=f]/S[fin=f]" end="2" begin="0" id="s0_sp2"/>
          <span terminal="s0_0" category="NP[mod=nm,case=nc,fin=f]" end="1" begin="0" id="s0_sp3"/>
          <span terminal="s0_1" category="(S[fin=f]/S[fin=f])\NP[mod=nm,case=nc,fin=f]" end="2" begin="1" id="s0_sp4"/>
          <span child="s0_sp6 s0_sp9" rule="&lt;" category="S[mod=nm,form=base,fin=f]" end="5" begin="2" id="s0_sp5"/>
          <span child="s0_sp7 s0_sp8" rule="&lt;" category="S[mod=nm,form=cont,fin=f]" end="4" begin="2" id="s0_sp6"/>
          <span terminal="s0_2" category="NP[mod=nm,case=nc,fin=f]" end="3" begin="2" id="s0_sp7"/>
          <span terminal="s0_3" category="S[mod=nm,form=cont,fin=f]\NP[mod=nm,case=nc,fin=f]" end="4" begin="3" id="s0_sp8"/>
          <span terminal="s0_4" category="S[mod=nm,form=base,fin=f]\S[mod=nm,form=cont,fin=f]" end="5" begin="4" id="s0_sp9"/>
          <span terminal="s0_5" category="S[mod=nm,form=base,fin=t]\S[mod=nm,form=base,fin=f]" end="6" begin="5" id="s0_sp10"/>
        </ccg>
        <ccg score="359.9289990067482" id="s0_ccg1" root="s0_sp11">
          <span child="s0_sp12 s0_sp21" rule="&lt;" category="S[mod=nm,form=base,fin=t]" end="6" begin="0" id="s0_sp11"/>
          <span child="s0_sp13 s0_sp20" rule="&lt;" category="S[mod=nm,form=base,fin=f]" end="5" begin="0" id="s0_sp12"/>
          <span child="s0_sp14 s0_sp17" rule="&gt;" category="S[mod=nm,form=cont,fin=f]" end="4" begin="0" id="s0_sp13"/>
          <span child="s0_sp15 s0_sp16" rule="&lt;" category="S[fin=f]/S[fin=f]" end="2" begin="0" id="s0_sp14"/>
          <span terminal="s0_0" category="NP[mod=nm,case=nc,fin=f]" end="1" begin="0" id="s0_sp15"/>
          <span terminal="s0_1" category="(S[fin=f]/S[fin=f])\NP[mod=nm,case=nc,fin=f]" end="2" begin="1" id="s0_sp16"/>
          <span child="s0_sp18 s0_sp19" rule="&lt;" category="S[mod=nm,form=cont,fin=f]" end="4" begin="2" id="s0_sp17"/>
          <span terminal="s0_2" category="NP[mod=nm,case=nc,fin=f]" end="3" begin="2" id="s0_sp18"/>
          <span terminal="s0_3" category="S[mod=nm,form=cont,fin=f]\NP[mod=nm,case=nc,fin=f]" end="4" begin="3" id="s0_sp19"/>
          <span terminal="s0_4" category="S[mod=nm,form=base,fin=f]\S[mod=nm,form=cont,fin=f]" end="5" begin="4" id="s0_sp20"/>
          <span terminal="s0_5" category="S[mod=nm,form=base,fin=t]\S[mod=nm,form=base,fin=f]" end="6" begin="5" id="s0_sp21"/>
        </ccg>
        <ccg score="302.90713411569595" id="s0_ccg2" root="s0_sp22">
          <span child="s0_sp23 s0_sp32" rule="&lt;" category="S[mod=nm,form=base,fin=t]" end="6" begin="0" id="s0_sp22"/>
          <span child="s0_sp24 s0_sp27" rule="&lt;" category="S[mod=nm,form=base,fin=f]" end="5" begin="0" id="s0_sp23"/>
          <span child="s0_sp25 s0_sp26" rule="&lt;" category="NP[mod=nm,case=ga,fin=f]" end="2" begin="0" id="s0_sp24"/>
          <span terminal="s0_0" category="NP[mod=nm,case=nc,fin=f]" end="1" begin="0" id="s0_sp25"/>
          <span terminal="s0_1" category="NP[mod=nm,case=ga,fin=f]\NP[mod=nm,case=nc,fin=f]" end="2" begin="1" id="s0_sp26"/>
          <span child="s0_sp28 s0_sp31" rule="&lt;B1" category="S[mod=nm,form=base,fin=f]\NP[mod=nm,case=ga,fin=f]" end="5" begin="2" id="s0_sp27"/>
          <span child="s0_sp29 s0_sp30" rule="&lt;" category="S[mod=nm,form=cont,fin=f]\NP[mod=nm,case=ga,fin=f]" end="4" begin="2" id="s0_sp28"/>
          <span terminal="s0_2" category="NP[mod=nm,case=nc,fin=f]" end="3" begin="2" id="s0_sp29"/>
          <span terminal="s0_3" category="(S[mod=nm,form=cont,fin=f]\NP[mod=nm,case=ga,fin=f])\NP[mod=nm,case=nc,fin=f]" end="4" begin="3" id="s0_sp30"/>
          <span terminal="s0_4" category="S[mod=nm,form=base,fin=f]\S[mod=nm,form=cont,fin=f]" end="5" begin="4" id="s0_sp31"/>
          <span terminal="s0_5" category="S[mod=nm,form=base,fin=t]\S[mod=nm,form=base,fin=f]" end="6" begin="5" id="s0_sp32"/>
        </ccg>
        <ccg score="295.9965096116066" id="s0_ccg3" root="s0_sp33 s0_sp36">
          <span child="s0_sp34 s0_sp35" rule="&lt;" category="S[fin=f]/S[fin=f]" end="2" begin="0" id="s0_sp33"/>
          <span terminal="s0_0" category="NP[mod=nm,case=nc,fin=f]" end="1" begin="0" id="s0_sp34"/>
          <span terminal="s0_1" category="(S[fin=f]/S[fin=f])\NP[mod=nm,case=nc,fin=f]" end="2" begin="1" id="s0_sp35"/>
          <span child="s0_sp37 s0_sp42" rule="&lt;" category="S[mod=nm,form=base,fin=t]" end="6" begin="2" id="s0_sp36"/>
          <span child="s0_sp38 s0_sp41" rule="&lt;" category="S[mod=nm,form=base,fin=f]" end="5" begin="2" id="s0_sp37"/>
          <span child="s0_sp39 s0_sp40" rule="&lt;" category="S[mod=nm,form=cont,fin=f]" end="4" begin="2" id="s0_sp38"/>
          <span terminal="s0_2" category="NP[mod=nm,case=nc,fin=f]" end="3" begin="2" id="s0_sp39"/>
          <span terminal="s0_3" category="S[mod=nm,form=cont,fin=f]\NP[mod=nm,case=nc,fin=f]" end="4" begin="3" id="s0_sp40"/>
          <span terminal="s0_4" category="S[mod=nm,form=base,fin=f]\S[mod=nm,form=cont,fin=f]" end="5" begin="4" id="s0_sp41"/>
          <span terminal="s0_5" category="S[mod=nm,form=base,fin=t]\S[mod=nm,form=base,fin=f]" end="6" begin="5" id="s0_sp42"/>
        </ccg>
        <ccg score="266.6446071267128" id="s0_ccg4" root="s0_sp43">
          <span child="s0_sp44 s0_sp53" rule="&lt;" category="S[mod=nm,form=base,fin=t]" end="6" begin="0" id="s0_sp43"/>
          <span child="s0_sp45 s0_sp48" rule="&gt;" category="S[mod=nm,form=base,fin=f]" end="5" begin="0" id="s0_sp44"/>
          <span child="s0_sp46 s0_sp47" rule="&lt;" category="S[fin=f]/S[fin=f]" end="2" begin="0" id="s0_sp45"/>
          <span terminal="s0_0" category="NP[mod=nm,case=nc,fin=f]" end="1" begin="0" id="s0_sp46"/>
          <span terminal="s0_1" category="(S[fin=f]/S[fin=f])\NP[mod=nm,case=nc,fin=f]" end="2" begin="1" id="s0_sp47"/>
          <span child="s0_sp49 s0_sp50" rule="&lt;" category="S[mod=nm,form=base,fin=f]" end="5" begin="2" id="s0_sp48"/>
          <span terminal="s0_2" category="NP[mod=nm,case=nc,fin=f]" end="3" begin="2" id="s0_sp49"/>
          <span child="s0_sp51 s0_sp52" rule="&lt;B1" category="S[mod=nm,form=base,fin=f]\NP[mod=nm,case=nc,fin=f]" end="5" begin="3" id="s0_sp50"/>
          <span terminal="s0_3" category="S[mod=nm,form=cont,fin=f]\NP[mod=nm,case=nc,fin=f]" end="4" begin="3" id="s0_sp51"/>
          <span terminal="s0_4" category="S[mod=nm,form=base,fin=f]\S[mod=nm,form=cont,fin=f]" end="5" begin="4" id="s0_sp52"/>
          <span terminal="s0_5" category="S[mod=nm,form=base,fin=t]\S[mod=nm,form=base,fin=f]" end="6" begin="5" id="s0_sp53"/>
        </ccg>
        <ccg score="246.66228300333023" id="s0_ccg5" root="s0_sp54">
          <span child="s0_sp55 s0_sp64" rule="&lt;" category="S[mod=nm,form=base,fin=t]" end="6" begin="0" id="s0_sp54"/>
          <span child="s0_sp56 s0_sp63" rule="&lt;" category="S[mod=nm,form=base,fin=f]" end="5" begin="0" id="s0_sp55"/>
          <span child="s0_sp57 s0_sp62" rule="&lt;" category="S[mod=nm,form=cont,fin=f]" end="4" begin="0" id="s0_sp56"/>
          <span child="s0_sp58 s0_sp61" rule="&gt;" category="NP[mod=nm,case=nc,fin=f]" end="3" begin="0" id="s0_sp57"/>
          <span child="s0_sp59 s0_sp60" rule="&lt;" category="NP[fin=f]/NP[fin=f]" end="2" begin="0" id="s0_sp58"/>
          <span terminal="s0_0" category="NP[mod=nm,case=nc,fin=f]" end="1" begin="0" id="s0_sp59"/>
          <span terminal="s0_1" category="(NP[fin=f]/NP[fin=f])\NP[mod=nm,case=nc,fin=f]" end="2" begin="1" id="s0_sp60"/>
          <span terminal="s0_2" category="NP[mod=nm,case=nc,fin=f]" end="3" begin="2" id="s0_sp61"/>
          <span terminal="s0_3" category="S[mod=nm,form=cont,fin=f]\NP[mod=nm,case=nc,fin=f]" end="4" begin="3" id="s0_sp62"/>
          <span terminal="s0_4" category="S[mod=nm,form=base,fin=f]\S[mod=nm,form=cont,fin=f]" end="5" begin="4" id="s0_sp63"/>
          <span terminal="s0_5" category="S[mod=nm,form=base,fin=t]\S[mod=nm,form=base,fin=f]" end="6" begin="5" id="s0_sp64"/>
        </ccg>
        <ccg score="238.37061220407486" id="s0_ccg6" root="s0_sp65 s0_sp68 s0_sp71 s0_sp72">
          <span child="s0_sp66 s0_sp67" rule="&lt;" category="S[fin=f]/S[fin=f]" end="2" begin="0" id="s0_sp65"/>
          <span terminal="s0_0" category="NP[mod=nm,case=nc,fin=f]" end="1" begin="0" id="s0_sp66"/>
          <span terminal="s0_1" category="(S[fin=f]/S[fin=f])\NP[mod=nm,case=nc,fin=f]" end="2" begin="1" id="s0_sp67"/>
          <span child="s0_sp69 s0_sp70" rule="&lt;" category="S[mod=nm,form=cont,fin=f]" end="4" begin="2" id="s0_sp68"/>
          <span terminal="s0_2" category="NP[mod=nm,case=nc,fin=f]" end="3" begin="2" id="s0_sp69"/>
          <span terminal="s0_3" category="S[mod=nm,form=cont,fin=f]\NP[mod=nm,case=nc,fin=f]" end="4" begin="3" id="s0_sp70"/>
          <span terminal="s0_4" category="S[mod=nm,form=base,fin=f]\S[mod=nm,form=cont,fin=f]" end="5" begin="4" id="s0_sp71"/>
          <span terminal="s0_5" category="S[mod=nm,form=base,fin=t]\S[mod=nm,form=base,fin=f]" end="6" begin="5" id="s0_sp72"/>
        </ccg>
        <ccg score="235.2004957795143" id="s0_ccg7" root="s0_sp73 s0_sp76">
          <span child="s0_sp74 s0_sp75" rule="&lt;" category="NP[mod=nm,case=ga,fin=f]" end="2" begin="0" id="s0_sp73"/>
          <span terminal="s0_0" category="NP[mod=nm,case=nc,fin=f]" end="1" begin="0" id="s0_sp74"/>
          <span terminal="s0_1" category="NP[mod=nm,case=ga,fin=f]\NP[mod=nm,case=nc,fin=f]" end="2" begin="1" id="s0_sp75"/>
          <span child="s0_sp77 s0_sp82" rule="&lt;" category="S[mod=nm,form=base,fin=t]" end="6" begin="2" id="s0_sp76"/>
          <span child="s0_sp78 s0_sp81" rule="&lt;" category="S[mod=nm,form=base,fin=f]" end="5" begin="2" id="s0_sp77"/>
          <span child="s0_sp79 s0_sp80" rule="&lt;" category="S[mod=nm,form=cont,fin=f]" end="4" begin="2" id="s0_sp78"/>
          <span terminal="s0_2" category="NP[mod=nm,case=nc,fin=f]" end="3" begin="2" id="s0_sp79"/>
          <span terminal="s0_3" category="S[mod=nm,form=cont,fin=f]\NP[mod=nm,case=nc,fin=f]" end="4" begin="3" id="s0_sp80"/>
          <span terminal="s0_4" category="S[mod=nm,form=base,fin=f]\S[mod=nm,form=cont,fin=f]" end="5" begin="4" id="s0_sp81"/>
          <span terminal="s0_5" category="S[mod=nm,form=base,fin=t]\S[mod=nm,form=base,fin=f]" end="6" begin="5" id="s0_sp82"/>
        </ccg>
        <ccg score="205.13470548391342" id="s0_ccg8" root="s0_sp83 s0_sp84 s0_sp85">
          <span terminal="s0_0" category="NP[mod=nm,case=nc,fin=f]" end="1" begin="0" id="s0_sp83"/>
          <span terminal="s0_1" category="NP[mod=nm,case=ga,fin=f]\NP[mod=nm,case=nc,fin=f]" end="2" begin="1" id="s0_sp84"/>
          <span child="s0_sp86 s0_sp91" rule="&lt;" category="S[mod=nm,form=base,fin=t]" end="6" begin="2" id="s0_sp85"/>
          <span child="s0_sp87 s0_sp90" rule="&lt;" category="S[mod=nm,form=base,fin=f]" end="5" begin="2" id="s0_sp86"/>
          <span child="s0_sp88 s0_sp89" rule="&lt;" category="S[mod=nm,form=cont,fin=f]" end="4" begin="2" id="s0_sp87"/>
          <span terminal="s0_2" category="NP[mod=nm,case=nc,fin=f]" end="3" begin="2" id="s0_sp88"/>
          <span terminal="s0_3" category="S[mod=nm,form=cont,fin=f]\NP[mod=nm,case=nc,fin=f]" end="4" begin="3" id="s0_sp89"/>
          <span terminal="s0_4" category="S[mod=nm,form=base,fin=f]\S[mod=nm,form=cont,fin=f]" end="5" begin="4" id="s0_sp90"/>
          <span terminal="s0_5" category="S[mod=nm,form=base,fin=t]\S[mod=nm,form=base,fin=f]" end="6" begin="5" id="s0_sp91"/>
        </ccg>
        <ccg score="185.43084102869034" id="s0_ccg9" root="s0_sp92 s0_sp95">
          <span child="s0_sp93 s0_sp94" rule="&lt;" category="NP[mod=nm,case=o,fin=f]" end="2" begin="0" id="s0_sp92"/>
          <span terminal="s0_0" category="NP[mod=nm,case=nc,fin=f]" end="1" begin="0" id="s0_sp93"/>
          <span terminal="s0_1" category="NP[mod=nm,case=o,fin=f]\NP[mod=nm,case=nc,fin=f]" end="2" begin="1" id="s0_sp94"/>
          <span child="s0_sp96 s0_sp101" rule="&lt;" category="S[mod=nm,form=base,fin=t]" end="6" begin="2" id="s0_sp95"/>
          <span child="s0_sp97 s0_sp100" rule="&lt;" category="S[mod=nm,form=base,fin=f]" end="5" begin="2" id="s0_sp96"/>
          <span child="s0_sp98 s0_sp99" rule="&lt;" category="S[mod=nm,form=cont,fin=f]" end="4" begin="2" id="s0_sp97"/>
          <span terminal="s0_2" category="NP[mod=nm,case=nc,fin=f]" end="3" begin="2" id="s0_sp98"/>
          <span terminal="s0_3" category="S[mod=nm,form=cont,fin=f]\NP[mod=nm,case=nc,fin=f]" end="4" begin="3" id="s0_sp99"/>
          <span terminal="s0_4" category="S[mod=nm,form=base,fin=f]\S[mod=nm,form=cont,fin=f]" end="5" begin="4" id="s0_sp100"/>
          <span terminal="s0_5" category="S[mod=nm,form=base,fin=t]\S[mod=nm,form=base,fin=f]" end="6" begin="5" id="s0_sp101"/>
        </ccg>
      </sentence>
    </sentences>
  </document>
</root>"##;
    let ccg: Root = from_reader(s.as_bytes()).unwrap();
    println!("{:#?}", ccg);
}
