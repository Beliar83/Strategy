using Microsoft.JSInterop;
using Microsoft.Xna.Framework;
using System;
using Strategy.Game;

namespace Strategy.Pages
{
    public partial class Index
    {
        Microsoft.Xna.Framework.Game _game;

        protected override void OnAfterRender(bool firstRender)
        {
            base.OnAfterRender(firstRender);

            if (firstRender)
            {
                JsRuntime.InvokeAsync<object>("initRenderJS", DotNetObjectReference.Create(this));
            }
        }

        [JSInvokable]
        public void TickDotNet()
        {
            // init game
            if (_game == null)
            {
                _game = new StrategyGame();
                _game.Run();
            }

            // run gameloop
            _game.Tick();
        }

    }
}
