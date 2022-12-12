fn main() {
    MainWindow::new().run();
}
sixtyfps::sixtyfps! {
    MemoryTile := Rectangle {
        callback clicked;
        property <bool> open_curtain;
        property <bool> solved;
        property <image> icon;
    
        height: 64px;
        width: 64px;
        background: solved ? #34CE57 : #ffffff;
        animate background { duration: 800ms; }
    
        Image {
            source: icon;
            width: parent.width;
            height: parent.height;
        }
    
        // Left curtain
        Rectangle {
            background: #DCDCDC;
            width: open_curtain ? 0px : (parent.width / 2);
            height: parent.height;
            animate width { duration: 250ms; easing: ease-in; }
        }
    
        // Right curtain
        Rectangle {
            background: #DCDCDC;
            x: open_curtain ? parent.width : (parent.width / 2);
            width: open_curtain ? 0px : (parent.width / 2);
            height: parent.height;
            animate width { duration: 250ms; easing: ease-in; }
            animate x { duration: 250ms; easing: ease-in; }
        }
    
        TouchArea {
            clicked => {
                // Delegate to the user of this element
                root.clicked();
            }
        }
    }
    MainWindow := Window {
        MemoryTile {
            icon: @image-url("icons_/monkey.png");
            clicked => {
                self.open_curtain = !self.open_curtain;
            }
        }
    }
}