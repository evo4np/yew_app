use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    count: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            count: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class ="container">
                <button onclick={ctx.link().callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.count }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

