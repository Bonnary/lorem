use ::std::env;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage:./{} <number of wordes>", args[0]);
        return;
    }
    let number = args[1].trim().parse::<u32>().ok().expect("Invalid number");
    let lorem ="Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec mauris ligula, sagittis non orci quis, pulvinar molestie elit. Suspendisse lacinia laoreet mi in dictum. Morbi ornare eleifend risus. Ut imperdiet sollicitudin laoreet. Nulla nunc turpis, facilisis sed lobortis eget, molestie a diam. Fusce metus leo, ullamcorper gravida risus quis, iaculis commodo quam. Praesent in mi elit. Sed malesuada est quam, et lacinia odio euismod vel. Donec scelerisque sollicitudin vestibulum. Sed scelerisque magna lacus, et ultricies velit scelerisque ac. Vestibulum et rhoncus quam.
                Duis dapibus eros nec feugiat finibus. Integer mollis turpis quis tortor congue lacinia. Donec elementum dapibus lectus et maximus. Nullam id nulla rutrum, mattis ex vitae, rutrum magna. Praesent imperdiet condimentum dolor ac accumsan. Suspendisse et ligula sapien. Phasellus viverra mauris eget lacus lacinia accumsan. Etiam non quam ut elit hendrerit posuere vitae iaculis massa. Curabitur elit erat, mollis ut turpis quis, tempus tincidunt metus. Ut lacus odio, tempor non tortor ut, viverra auctor tellus. Nullam mattis in dolor vel tincidunt. Fusce in mollis arcu. Donec rhoncus nisl at tempus tincidunt. Ut non accumsan sem.
                Vestibulum leo ipsum, dapibus in justo aliquam, efficitur egestas mauris. Integer rutrum nisi in purus mattis aliquam. Morbi non tortor tempor, interdum lectus eget, luctus nunc. Nam aliquam semper quam, non commodo nibh dapibus sit amet. Cras a tortor dui. Quisque eget gravida nunc, nec fringilla est. Etiam non nulla consectetur, pharetra est eget, vehicula erat. Proin sit amet leo sem. Nunc ut enim nec metus interdum facilisis. Nullam dolor erat, maximus at pharetra eget, dapibus quis risus. Aenean elementum ultricies commodo.
                Praesent posuere sollicitudin pharetra. Donec dictum erat et neque fringilla, nec finibus lectus sollicitudin. Duis dapibus sem a ultrices finibus. Morbi ultrices urna vitae mi rutrum tempus. Vestibulum aliquam nunc et arcu accumsan gravida sit amet scelerisque enim. Aenean facilisis eros in cursus dictum. Morbi porttitor tincidunt risus id cursus. Mauris nec euismod neque, in ullamcorper elit. Maecenas finibus hendrerit laoreet. Aliquam urna leo, pretium sed quam et, aliquam interdum mauris. Mauris dictum massa in venenatis elementum. In posuere risus velit, quis tempus quam cursus at. Aenean mi enim, molestie interdum urna a, mattis vestibulum sapien. Suspendisse mollis condimentum erat id interdum. Fusce fermentum dui in metus faucibus, non porttitor turpis venenatis. Duis sed odio et mi lobortis vehicula in a risus.
                Praesent leo neque, vulputate non luctus quis, sodales ac ipsum. Aenean turpis arcu, consequat at metus porttitor, elementum faucibus diam. Aliquam euismod velit eget semper efficitur. Nam tincidunt vulputate magna, et rhoncus urna tincidunt ut. Etiam vel enim a metus bibendum cursus. In vel eros sed risus convallis aliquet non et sem. Sed non consequat leo, eu blandit nibh. Vestibulum convallis, odio at tempus euismod, odio arcu lacinia risus, sit amet rutrum mauris libero id augue. Curabitur sodales eros nibh, quis ultrices lacus aliquet et. Nullam aliquam porta quam ut dignissim.".split(" ");

    let lorem_array: Vec<&str> = lorem.collect();
    let mut lorem_word = String::new();

    for _ in 0..number {
        let ranum = rand::thread_rng().gen_range(0..lorem_array.len());
        lorem_word.push_str(lorem_array[ranum]);
        lorem_word.push_str(" ");
    }
    println!("{}", lorem_word);
}
