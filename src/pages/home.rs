static HOME_TEMPLATE: &'static str = r#"
<!DOCTYPE html>
<html lang="en-us">
    <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width" />
        <title>Slack-translate analytics</title>
<style>
* {
    box-sizing: border-box;
}
html {
    background: #292929;
    color: #EBEBEB;
    font-family: sans-serif;
}
body {
    margin: 0;
}
a:hover, a:visited, a:link, a:active {
    text-decoration: none;
    color: #EBEBEB;
}
a:hover {
    border-bottom: 2px solid #FFC300;
}
.width-container {
    margin: 0 auto;
    max-width 800px;
    width: 90%;
}
.title-container {
    margin-top: 80px;
    margin-bottom: 40px;
    font-size: 2rem;
    font-weight: 700;
}
.log-container {
}
.tl-outer {
    display: flex;
    flex-direction: column;
    margin: 20px 0;
    width: 100%;
    background: #1D1E1F;
    border-radius: 24px;
    padding: 30px 40px;
    box-shadow: 0 3px 6px rgba(0,0,0,0.16), 0 3px 6px rgba(0,0,0,0.23);
}
.tl-label-outer {
    display: flex;
    gap: 30px;
    justify-content: space-between;
    font-size: 1.1rem;
}
.tl-translation-outer {
    font-size: 1.25rem;
    background: #292929;
    border-radius: 8px;
    line-height: 1.5;
    padding-right: 8px;
}
.tl-outer > div:nth-child(2) {
    margin-top: 20px;
    margin-bottom: 10px;
}
.tl-type-label {
    display: inline-block;
    width: 12%;
    padding: 4px 0 4px 8px;
    border-radius: 8px 0 0 8px;
    margin-right: 20px;
    vertical-align: top;
    display: inline-block;
}
.tl-content {
    display: inline-block;
    width: calc(88% - 28px);
    padding: 4px 0px;
}
.lang-flow {
    background: #292929;
    line-height: 1.5;
    border-radius: 8px;
    padding-right: 6px;
    overflow: hidden;
    font-weight: 700;
}
.tl-lang-original {
    padding: 4px 0px 4px 6px;
    margin-right: 25px;
    border-radius: 8px 0 0 8px;
    background: #00DEE2;
    position: relative;
    z-index: 0;
    overflow: hidden;
    color: #292929;
}
.tl-lang-original:before {
    content: '';
    position: absolute;
    display: inline-block;
    z-index: -1;
    left: 85%;
    top: -18%;
    width: 0;
    height: 0;
    border-top: 20px solid transparent;
    border-bottom: 20px solid transparent;
    border-left: 20px solid #00DEE2;
}
.original {
    background: #FF1BB9;
}
.translated {
    background: #B661FF; 
}
.footer-container {
    margin-top: 120px;
    margin-bottom: 80px;
    font-size: 1.25rem;
}
.overview-container {
    display: flex;
    gap: 20px;
}
.overview-bg {
    padding: 30px;
    background: #1D1E1F;
    border-radius: 24px;
}
.totals {
    flex: 1;
}
.days {
    flex: 2;
}
.lcg-outer-container {
    display: flex;
    flex-direction: column;
    gap: 20px;
}
.lcg-container {
    line-height: 1.5;
    border-radius: 8px;
    background: #292929;
    display: flex;
    justify-content: space-between;
    position: relative;
}
.lcg-lang-text {
    line-height: 1.5;
    font-size: 1.1rem;
    display: inline-block;
    float: left;
    overflow: unset;
}
.lcg-lang-text:before {
    left: 93%;
    top: -10%;
}
.lcg-res-text {
    padding: 4px 0;
}
.lcg-ja, .lcg-en {
    float: left;
    line-height: 1.5;
    display: inline-block;
    align-self: flex-start;
}
.lcg-ja-bar, .lcg-en-bar {
    position: absolute;
    height: 100%;
}
.lcg-lang-text-en {
    background: #FFC300;
}
.lcg-lang-text-en:before {
    border-left: 20px solid #FFC300;
}
.lcg-lang-text-ja {
    background: #FF8500;
}
.lcg-lang-text-ja:before {
    border-left: 20px solid #FF8500;
}
.lcg-tl-count {
    float: left;
    display: inline-block;
    padding: 4px 0;
    font-size: 1.1rem;
    line-height: 1.5;
}
.lcg-tl-total {
    padding: 4px 0;
    font-size: 1.1rem;
    line-height: 1.5;
}
.lcg-total {
    font-size: 3rem;
    font-weight: 700;
    padding-right: 10px;
    line-height: 1;
    color: #00DEE2;
}
.lcg-lang-container {
    display: flex;
    justify-content: space-between;
}
.lcg-lang-count {
    font-weight: 700;
    font-size: 3rem;
    line-height: 0.1;
    padding-right: 10px;
}
.lcg-ja-bar {
    right: 0;
    background: #FF8500;
    border-radius: 0 8px 8px 0;
    border-left: 3px solid #292929;
}
.lcg-en-bar {
    background: #FFC300;
    border-radius: 8px 0 0 8px;
    border-right: 3px solid #292929;
}
.placeholder {
    padding: 4px 0;
    color: rgba(0, 0, 0, 0.5);
}
    
</style>
    </head>
    <body>
        <section class="title-container width-container">
            <h1>Slack-translate analytics</h1>
        </section>
        <section class="overview-container width-container">
            <div class="totals overview-bg">
                <div class="lcg-outer-container">
                    <div class="lcg-tl-total"><span class="lcg-total">{tl_total}</span> total translations</div>
                    <div class="lcg-lang-container">
                        <div class="lcg-tl-count"><span class="lcg-lang-count">{en_ja_tl}</span> translations from English</div>
                        <div class="lcg-en tl-label-inner lang-flow">
                            <span class="lcg-lang-text lcg-lang-text-en tl-lang-original">EN</span>
                            <span class="lcg-lang-text lcg-res-text">JA</span>
                        </div>
                    </div>
                    <div class="lcg-lang-container">
                        <div class="lcg-tl-count"><span class="lcg-lang-count">{ja_en_tl}</span> translations from Japanese</div>
                        <div class="lcg-ja tl-label-inner lang-flow">
                            <span class="lcg-lang-text lcg-lang-text-ja tl-lang-original">JA</span>
                            <span class="lcg-lang-text lcg-res-text">EN</span>
                        </div>
                    </div>
                    <div class="lcg-container">
                        <div class="placeholder">Placeholder</div>
                        <div class="lcg-en-bar" style="width:{Value}%;"></div>
                        <div class="lcg-ja-bar" style="width:{Value}%;"></div>
                    </div>
                </div>
            </div>
            <div class="days overview-bg">
            </div>
        </section>
        <section class="log-container width-container">
            {TranslationLog}
        </section>
        <section class="footer-container width-container">
            <a href="https://github.com/JesseLeung97/slack-translate" target="_blank">Check out the source code on Github</a>
        </section>
    </body>
</html>
"#;
