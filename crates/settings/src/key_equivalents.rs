use collections::HashMap;

// On some keyboards (e.g. German QWERTZ) it is not possible to type the full ASCII range
// without using option. This means that some of our built in keyboard shortcuts do not work
// for those users.
//
// The way macOS solves this problem is to move shortcuts around so that they are all reachable,
// even if the mnemoic changes. https://developer.apple.com/documentation/swiftui/keyboardshortcut/localization-swift.struct
//
// For example, cmd-> is the "switch window" shortcut because the > key is right above tab.
// To ensure this doesn't cause problems for shortcuts defined for a QWERTY layout, apple moves
// any shortcuts defined as cmd-> to cmd-:. Coincidentally this s also the same keyboard position
// as cmd-> on a QWERTY layout.
//
// Another example is cmd-[ and cmd-], as they cannot be typed without option, those keys are remapped to cmd-ö
// and cmd-ä. These shortcuts are not in the same position as a QWERTY keyboard, because on a QWERTZ keyboard
// the + key is in the way; and shortcuts bound to cmd-+ are still typed as cmd-+ on either keyboard (though the
// specific key moves)
//
// As far as I can tell, there's no way to query the mappings Apple uses except by rendering a menu with every
// possible key combination, and inspecting the UI to see what it rendered. So that's what we did...
//
// These mappings were generated by running https://github.com/ConradIrwin/keyboard-inspector, tidying up the
// output to remove languages with no mappings and other oddities, and converting it to a less verbose representation with:
//  jq -s 'map(to_entries | map({key: .key, value: [(.value | to_entries | map(.key) | join("")), (.value | to_entries | map(.value) | join(""))]}) | from_entries) | add'
// From there I used multi-cursor to produce this match statement.
#[cfg(target_os = "macos")]
pub fn get_key_equivalents(layout: &str) -> Option<HashMap<char, char>> {
    let (from, to) = match layout {
        "com.apple.keylayout.Welsh" => ("#", "£"),
        "com.apple.keylayout.Turkmen" => ("qc]Q`|[XV\\^v~Cx}{", "äçöÄžŞňÜÝş№ýŽÇüÖŇ"),
        "com.apple.keylayout.Turkish-QWERTY-PC" => (
            "$\\|`'[}^=.#{*+:/~;)(@<,&]>\"",
            "+,;<ığÜ&.ç^Ğ(:Ş*>ş=)'Öö/üÇI",
        ),
        "com.apple.keylayout.Sami-PC" => (
            "}*x\"w[~^/@`]{|<)>W(\\X=Qq&':;",
            "Æ(čŊšøŽ&´\"žæØĐ;=:Š)đČ`Áá/ŋÅå",
        ),
        "com.apple.keylayout.LatinAmerican" => {
            ("[^~>`(<\\@{;*&/):]|='}\"", "{&>:<);¿\"[ñ(/'=Ñ}¡*´]¨")
        }
        "com.apple.keylayout.IrishExtended" => ("#", "£"),
        "com.apple.keylayout.Icelandic" => ("[}=:/'){(*&;^|`\"\\>]<~@", "æ´*Ð'ö=Æ)(/ð&Þ<Öþ:´;>\""),
        "com.apple.keylayout.German-DIN-2137" => {
            ("}~/<^>{`:\\)&=[]@|;#'\"(*", "Ä>ß;&:Ö<Ü#=/*öä\"'ü§´`)(")
        }
        "com.apple.keylayout.FinnishSami-PC" => {
            (")=*\"\\[@{:>';/<|~(]}^`&", "=`(ˆ@ö\"ÖÅ:¨å´;*>)äÄ&</")
        }
        "com.apple.keylayout.FinnishExtended" => {
            ("];{`:'*<~=/}\\|&[\"($^)>@", "äåÖ<Å¨(;>`´Ä'*/öˆ)€&=:\"")
        }
        "com.apple.keylayout.Faroese" => ("}\";/$>^@~`:&[*){|]=(\\<'", "ÐØæ´€:&\"><Æ/å(=Å*ð`)';ø"),
        "com.apple.keylayout.Croatian-PC" => {
            ("{@~;<=>(&*['|]\":/}^`)\\", "Š\">č;*:)/(šćŽđĆČ'Đ&<=ž")
        }
        "com.apple.keylayout.Croatian" => ("{@;<~=>(&*['|]\":}^)\\`", "Š\"č;>*:)'(šćŽđĆČĐ&=ž<"),
        "com.apple.keylayout.Azeri" => (":{W?./\"[}<]|,>';w", "IÖÜ,ş.ƏöĞÇğ/çŞəıü"),
        "com.apple.keylayout.Albanian" => ("\\'~;:|<>`\"@", "ë@>çÇË;:<'\""),
        "com.apple.keylayout.SwissFrench" => (
            ":@&'~^)$;\"][\\/#={!|*+`<(>}",
            "ü\"/^>&=çè`àé$'*¨ö+£(!<;):ä",
        ),
        "com.apple.keylayout.Swedish" => ("(]\\\"~$`^{|/>*:;<)&=[}'@", ")ä'^>€<&Ö*´:(Åå;=/`öÄ¨\""),
        "com.apple.keylayout.Swedish-Pro" => {
            ("/^*`'{|)$>&<[\\;(~\"}@]:=", "´&(<¨Ö*=€:/;ö'å)>^Ä\"äÅ`")
        }
        "com.apple.keylayout.Spanish" => ("|!\\<{[:;@`/~].'>}\"^", "\"¡'¿Ññº´!<.>;ç`Ç:¨/"),
        "com.apple.keylayout.Spanish-ISO" => (
            "|~`]/:)(<&^>*;#}\"{.\\['@",
            "\"><;.º=)¿/&Ç(´·not found¨Ñç'ñ`\"",
        ),
        "com.apple.keylayout.Portuguese" => (")`/'^\"<];>[:{@}(&*=~", "=<'´&`;~º:çªÇ\"^)/(*>"),
        "com.apple.keylayout.Italian" => (
            "*7};8:!5%(1&4]^\\6)32>.</0|$,'{[`\"~9#@",
            "8)*ò£!1ç59&7($6§è0'\"/:.,é°4;ù^ì<%>à32",
        ),
        "com.apple.keylayout.Italian-Pro" => {
            ("/:@[]'\\=){;|#<\"(*^&`}>~", "'é\"òàìù*=çè§£;^)(&/<°:>")
        }
        "com.apple.keylayout.Irish" => ("#", "£"),
        "com.apple.keylayout.German" => ("=`#'}:)/\"^&]*{;|[<(>~@\\", "*<§´ÄÜ=ß`&/ä(Öü'ö;):>\"#"),
        "com.apple.keylayout.French" => (
            "*}7;8:!5%(1&4]\\^6)32>.</0|${'[`\"~9#@",
            "8*è)!°1(59&7'$`6§0\"é/;.:à£4¨ù^<%>ç32",
        ),
        "com.apple.keylayout.French-numerical" => (
            "|!52;][>&@\"%'{)<~7.1/^(}*8#0$9`6\\3:4",
            "£1(é)$^/72%5ù¨0.>è;&:69*8!3à4ç<§`\"°'",
        ),
        "com.apple.keylayout.French-PC" => (
            "!&\"_$}/72>8]#:31)*<%4;6\\-{['@(0|5.`9~^",
            "17%°4£:èé/_$3§\"&08.5'!-*)¨^ù29àμ(;<ç>6",
        ),
        "com.apple.keylayout.Finnish" => ("/^*`)'{|$>&<[\\~;(\"}@]:=", "´&(<=¨Ö*€:/;ö'>å)^Ä\"äÅ`"),
        "com.apple.keylayout.Danish" => ("=[;'`{}|>]*^(&@~)<\\/$\":", "`æå¨<ÆØ*:ø(&)/\">=;'´€^Å"),
        "com.apple.keylayout.Canadian-CSA" => ("\\?']/><[{}|~`\"", "àÉèçé\"'^¨ÇÀÙùÈ"),
        "com.apple.keylayout.British" => ("#", "£"),
        "com.apple.keylayout.Brazilian-ABNT2" => ("\"|~?`'/^\\", "`^\"Ç'´ç¨~"),
        "com.apple.keylayout.Belgian" => (
            "`3/*<\\8>7#&96@);024(|'1\":$[~5.%^}]{!",
            "<\":8.`!/è37ç§20)àé'9£ù&%°4^>(;56*$¨1",
        ),
        "com.apple.keylayout.Austrian" => ("/^*`'{|)>&<[\\;(~\"}@]:=#", "ß&(<´Ö'=:/;ö#ü)>`Ä\"äÜ*§"),
        "com.apple.keylayout.Slovak-QWERTY" => (
            "):9;63'\"]^/+@~>`?<!#5&${2}%*18(704[",
            "0\"íôžš§!ä6'%2Ň:ňˇ?13ť74ÚľÄ58+á9ýéčú",
        ),
        "com.apple.keylayout.Slovak" => (
            "!$`10&:#4^*~{%5')}6/\"[8]97?;<@23>(+",
            "14ň+é7\"3č68ŇÚ5ť§0Äž'!úáäíýˇô?2ľš:9%",
        ),
        "com.apple.keylayout.Polish" => (
            "&)|?,%:;^}]_{!+#(*`/[~<\"$.>'@=\\",
            ":\"$Ż.+Łł=)(ćź§]!/_<żó>śę?,ńą%[;",
        ),
        "com.apple.keylayout.Lithuanian" => ("+#&=!%1*@73^584$26", "ŽĘŲžĄĮąŪČųęŠįūėĖčš"),
        "com.apple.keylayout.Hungarian" => (
            "}(*@\"{=/|;>'[`<~\\!$&0#:]^)+",
            "Ú)(\"ÁŐóüŰé:áőíÜÍű'!=ö+Éú/ÖÓ",
        ),
        "com.apple.keylayout.Hungarian-QWERTY" => (
            "=]#>@/&<`0')~(\\!:*;$\"+^{|}[",
            "óú+:\"ü=ÜíöáÖÍ)ű'É(é!ÁÓ/ŐŰÚő",
        ),
        "com.apple.keylayout.Czech-QWERTY" => (
            "9>0[2()\"}@]46%5;#8{*7^~+!3?&'<$/1`:",
            "í:éúě90!(2)čž5řů3áÚ8ý6`%1šˇ7§?4'+¨\"",
        ),
        "com.apple.keylayout.Maltese" => ("[`}{#]~", "ġżĦĠ£ħŻ"),
        "com.apple.keylayout.Turkish" => (
            "|}(#>&^-/`$%@]~*,[\"<_.{:'\\)",
            "ÜI%\"Ç)/ş.<'(*ı>_öğ-ÖŞçĞ$,ü:",
        ),
        "com.apple.keylayout.Turkish-Standard" => {
            ("|}(#>=&^`@]~*,;[\"<.{:'\\)", "ÜI)^;*'&ö\"ıÖ(.çğŞ:,ĞÇşü=")
        }
        "com.apple.keylayout.NorwegianSami-PC" => {
            ("\"}~<`&>':{@*^|\\)=([]/;", "ˆÆ>;</:¨ÅØ\"(&*@=`)øæ´å")
        }
        "com.apple.keylayout.Serbian-Latin" => {
            (";\\@>&'<]\"|(=}^)`[~:*{", "čž\":'ć;đĆŽ)*Đ&=<š>Č(Š")
        }
        "com.apple.keylayout.Slovenian" => ("]`^@)&\":'*=<{;}(~>\\|[", "đ<&\"='ĆČć(*;ŠčĐ)>:žŽš"),
        "com.apple.keylayout.SwedishSami-PC" => {
            ("@=<^|`>){'&\"}]~[/:*\\(;", "\"`;&*<:=Ö¨/ˆÄä>ö´Å(@)å")
        }
        "com.apple.keylayout.SwissGerman" => (
            "={#:\\}!(+]/<\";$'`*[>&^~@)|",
            "¨é*è$à+)!ä';`üç^<(ö:/&>\"=£",
        ),
        "com.apple.keylayout.Hawaiian" => ("'", "ʻ"),
        "com.apple.keylayout.NorthernSami" => (
            ":/[<{X\"wQx\\(;~>W}`*@])'^|=q&",
            "Å´ø;ØČŊšÁčđ)åŽ:ŠÆž(\"æ=ŋ&Đ`á/",
        ),
        "com.apple.keylayout.USInternational-PC" => ("^~", "ˆ˜"),
        "com.apple.keylayout.NorwegianExtended" => ("^~", "ˆ˜"),
        "com.apple.keylayout.Norwegian" => ("`'~\"\\*|=/@)[:}&><]{(^;", "<¨>^@(*`´\"=øÅÆ/:;æØ)&å"),
        "com.apple.keylayout.ABC-QWERTZ" => {
            ("\"}~<`>'&#:{@*^|\\)=(]/;[", "`Ä>;<:´/§ÜÖ\"(&'#=*)äßüö")
        }
        "com.apple.keylayout.ABC-AZERTY" => (
            ">[$61%@7|)&8\":}593(.4^<!{`2]\\#;~*/'0",
            "/^4§&52è£07!%°*(ç\"9;'6.1¨<é$`3)>8:ùà",
        ),
        "com.apple.keylayout.Czech" => (
            "(7*#193620?/{)@~!$8+;:%4\">`^]&5}[<'",
            "9ý83+íšžěéˇ'Ú02`14á%ů\"5č!:¨6)7ř(ú?§",
        ),
        "com.apple.keylayout.Brazilian-Pro" => ("^~", "ˆ˜"),
        _ => {
            return None;
        }
    };
    debug_assert!(from.chars().count() == to.chars().count());

    Some(HashMap::from_iter(from.chars().zip(to.chars())))
}

#[cfg(not(target_os = "macos"))]
pub fn get_key_equivalents(_layout: &str) -> Option<HashMap<char, char>> {
    None
}