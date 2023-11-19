use crate::base::BaseFlow;

pub struct Choice {}
impl BaseFlow for Choice {
    fn start(&self) -> bool {
        let selected: usize = self.inquire_num(
            "Which function would you like to run?\n\n\
                1. Get today's releases.\n\
                2. Get week's most popular anime.\n\
                3. Get today's featured anime articles\n\
                4. Get today's popular wallpapers\n",
        );

        println!("mode: {:?}", selected.to_string());
        self.exec_selected_mode(self.get_selected_mode(selected));

        return true;
    }
}
