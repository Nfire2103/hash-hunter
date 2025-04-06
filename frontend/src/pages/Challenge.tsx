import { HeaderChallenge } from '@/components/HeaderChallenge';

export const Challenge = () => {
  return (
    <div className="min-h-screen flex flex-col">
      <HeaderChallenge />
      <div className="flex-grow flex flex-col items-center justify-center p-4">
        <h1 className="text-3xl font-bold ">Create an SPL Token with Rust</h1>
        <div className="w-3xl mb-4">
          <p className="text-lg mb-4">
            Particularly, these standards are set around the three key things
            every web developer needs to have some familiarity with - HTML, CSS,
            and JavaScript.
          </p>
          <p className="text-lg mb-4">
            HTML is the language used to visually place elements on your screen.
            Things like this paragraph you’re reading right now, things like
            buttons, things like a dropdown menu. You visually place elements
            across the screen using HTML.
          </p>
          <p className="text-lg mb-4">
            CSS is a styling language used to add styles and your own custom
            flair to these elements. By default, HTML elements look boring and
            plain. Remember those old 90s websites? Yeah, that was plain HTML.
            CSS allows you to customize things about HTML elements. Such as
            making a button round instead of a rectangle, changing the font of a
            paragraph, having some bold text or underlined text, and so on.
          </p>
          <p className="text-lg mb-4">
            And finally, to tie it all together comes JavaScript. JavaScript is
            arguably the most important aspect of building on web technologies.
            It is a fully functional programming language that is used to add
            real functionality to your website.
          </p>
          <p className="text-lg mb-4">
            With HTML and CSS, you can place elements on a screen and make them
            look nice - but they won’t actually do anything. Your button
            wouldn’t actually do anything if you click it, more posts on
            Instagram will not load as you kept scrolling down, and so on.
          </p>
          <p className="text-lg mb-4">
            JavaScript allows you to add real interactivity and functionality to
            your websites. It is, without a doubt, the language of the web - and
            you will find that most of the lessons here on LearnWeb3 will use
            JavaScript in one way or the other. A website without JavaScript has
            no functionality other than letting you scroll up and down and click
            on links.
          </p>
        </div>
      </div>
    </div>
  );
};
