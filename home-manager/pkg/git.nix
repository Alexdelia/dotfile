{
  programs.git = {
    enable = true;

    extraConfig = {
      user.useConfigOnly = true;

      user.self.name = "Alexdelia";
      user.self.email = "alexandre.delille.57@gmail.com";

      user.school.name = "adelille";
      user.school.email = "adelille@student.42.fr";

      user.work.name = "Alexandre Delille";
      user.work.email = "alexandre@terros.io";
    };
  };
}
