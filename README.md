# Nvim External Command

Externally change nvim colorscheme or send a command to nvim

There is nothing much to this, it just uses the api(nvim_command) to send Nvim an external command.

Allowing you to change the theme of known opened instances of nvim through their server api / unix socket.

![demo](/doc/demo.gif)

Clone and Install, make sure cargo bin is in your $PATH
```bash
cargo install --path .
```


If you just want set the colorscheme run the `theme` subcommand
```bash
nvim-external-command theme --name=kanagawa-dragon
```


If you need to do more things, create a nvim user command(see example below) and call it with the `exec` subcommand
```bash
nvim-external-command exec --command='ToggleBackground'
```


Nvim user command example setting theme and lualine
```lua
-- toggle_background.lua
-- call from somewhere: require('.......toggle_background).setup()

local C = {}

function C.set_light_background()
    vim.cmd([[
        let ayucolor="light"
        set background=light
        colorscheme ayu
    ]])
    require("lualine").setup({
        options = {
            theme  = "ayu_light"
        },
    })
end

function C.set_dark_background()
    vim.cmd([[
        set background=dark
        colorscheme kanagawa-dragon
    ]])
    require("lualine").setup({
        options = {
            theme  = "jellybeans"
        },
    })
end

function C.toggle()
    if (vim.o.background == 'dark') then
        C.set_light_background()
    else
        C.set_dark_background()
    end
end


function C.setup()
    -- create a user command for toggling the background and colorscheme
    vim.api.nvim_create_user_command(
        'ToggleBackground',
        function()
            C.toggle()
        end,
        { nargs = 0 }
    )

    vim.api.nvim_create_user_command(
        'SetDarkBackground',
        function()
            C.set_dark_background()
        end,
        { nargs = 0 }
    )

    vim.api.nvim_create_user_command(
        'SetLightBackground',
        function()
            C.set_light_background()
        end,
        { nargs = 0 }
    )

    -- vim.cmd.ToggleBackground
    vim.api.nvim_set_keymap("n", "<F6>", ':ToggleBackground<CR>', { noremap = true })

    -- start a random server matching the nvim-external-command prefix
    vim.cmd([[
        let g:random = rand(srand()) % 10000000000000
        -- '/tmp/nvim-ec.' prefix needs to match the tmp_prefix from nvim-external-command
        call serverstart('/tmp/nvim-ec.' .. g:random)
    ]])
end

return C
```

If your intention is only to toggle the background and if you don't mind changing after changing alacritty for example, you can just create a key mapping calling what you need, but you would need to call it with all nvim opened instances, while this does it for all.
```lua
vim.api.nvim_set_keymap("n", "<F6>", ':ToggleBackground<CR>', { noremap = true })
```

With all of that in place you can also now toggle alacritty or the terminal of your choice if supported.
```bash
# assumes this file alacritty and nvim is in ${XDG_CONFIG_HOME} and itself lives at ${XDG_CONFIG_HOME}/bin
# otherwise customise below paths to your liking

base_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

dark="kanagawa_dragon"
light="ayu_light"

toggle_light_dark() {
    # if it is the light one change to dark and vice-versa
    rg "^colors: \*${light}" ${base_dir}/../alacritty/alacritty.yml &> /dev/null
    if [[ $? -eq 0 ]]; then
        # alacritty
        sed -i "s#^colors: \*${light}#colors: \*${dark}#g" ${base_dir}/../alacritty/alacritty.yml
        # nvim
        nvim-external-command exec --command='SetDarkBackground'
    else
        # alacritty
        sed -i "s#^colors: \*${dark}#colors: \*${light}#g" ${base_dir}/../alacritty/alacritty.yml
        # nvim
        nvim-external-command exec --command='SetLightBackground'
    fi
}

toggle_light_dark
```


