#include "LuzumiDesktopMenu.h"

#include <QCursor>
#include <QDBusConnection>
#include <QDBusMessage>
#include <QDBusPendingCall>
#include <QDebug>
#include <QDir>
#include <QClipboard>
#include <QApplication>
#include <QFile>
#include <QFileInfo>
#include <QFont>
#include <QMimeData>
#include <QUrl>
#include <QInputDialog>
#include <QMenu>
#include <QPainter>
#include <QProcess>
#include <QStandardPaths>
#include <QSvgRenderer>
#include <KPluginFactory>

K_PLUGIN_CLASS_WITH_JSON(LuzumiDesktopMenu, "metadata.json")

// ── Luzumi SVG icons (16x16 viewBox, #cad3f5 color) ──

static QIcon svgIcon(const char *svgContent, const QColor &color = QColor(202, 211, 245))
{
    QString svg = QStringLiteral(
        "<svg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 16 16' fill='none'>"
        "%1</svg>"
    ).arg(QString::fromUtf8(svgContent).replace(QStringLiteral("currentColor"), color.name()));

    QSvgRenderer renderer(svg.toUtf8());
    QPixmap pix(32, 32); // 2x for HiDPI
    pix.fill(Qt::transparent);
    QPainter painter(&pix);
    renderer.render(&painter);
    painter.end();

    return QIcon(pix);
}

// Icon SVG fragments matching Luzumi's icon set
struct Icons {
    static QIcon folder()    { return svgIcon(R"(<path d="M2 4.5C2 3.67 2.67 3 3.5 3H6.17C6.7 3 7.2 3.21 7.59 3.59L8.41 4.41C8.8 4.79 9.3 5 9.83 5H12.5C13.33 5 14 5.67 14 6.5V11.5C14 12.33 13.33 13 12.5 13H3.5C2.67 13 2 12.33 2 11.5V4.5Z" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/>)"); }
    static QIcon file()      { return svgIcon(R"(<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>)"); }
    static QIcon paste()     { return svgIcon(R"(<rect x="4" y="1" width="8" height="2" rx="0.5" stroke="currentColor" stroke-width="1.2" fill="none"/><rect x="3" y="3" width="10" height="11" rx="1.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".08"/>)"); }
    static QIcon terminal()  { return svgIcon(R"(<rect x="1.5" y="2.5" width="13" height="11" rx="2" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".08"/><path d="M4.5 6L7 8.5L4.5 11" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round" fill="none"/><path d="M8.5 11H11.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>)"); }
    static QIcon app()       { return svgIcon(R"(<rect x="2" y="2" width="12" height="12" rx="3" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><circle cx="8" cy="8" r="2.5" stroke="currentColor" stroke-width="1.2" fill="none"/>)"); }
    static QIcon lock()      { return svgIcon(R"(<rect x="3" y="7" width="10" height="7" rx="1.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><path d="M5 7V5a3 3 0 016 0v2" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/>)"); }
    static QIcon logout()    { return svgIcon(R"(<path d="M9 2H12.5C13.33 2 14 2.67 14 3.5V12.5C14 13.33 13.33 14 12.5 14H9" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/><path d="M6 8H1.5M1.5 8L3.5 6M1.5 8L3.5 10" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round" fill="none"/>)"); }
    static QIcon wallpaper() { return svgIcon(R"(<rect x="2" y="3" width="12" height="10" rx="1.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".08"/><circle cx="5.5" cy="6.5" r="1.5" stroke="currentColor" stroke-width="1" fill="currentColor" opacity=".2"/><path d="M2 11L5.5 8L8 10L10.5 7.5L14 11" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round" fill="none"/>)"); }
    static QIcon display()   { return svgIcon(R"(<rect x="2" y="2" width="12" height="9" rx="1.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".08"/><path d="M6 13H10" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/><path d="M8 11V13" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>)"); }
};

// Luzumi-style dark theme stylesheet
static const char *MENU_STYLE = R"(
QMenu {
    background-color: rgba(22, 23, 36, 245);
    border: 1px solid rgba(255, 255, 255, 25);
    border-radius: 12px;
    padding: 5px;
    font-size: 13px;
    color: #cad3f5;
}
QMenu::item {
    padding: 6px 24px 6px 12px;
    border-radius: 8px;
    margin: 1px 4px;
    color: #cad3f5;
}
QMenu::item:selected {
    background-color: rgba(202, 211, 245, 20);
}
QMenu::item:disabled {
    color: #6e738d;
    font-size: 10px;
    font-weight: bold;
    padding: 4px 12px 2px 12px;
    letter-spacing: 0.5px;
}
QMenu::separator {
    height: 1px;
    background: rgba(255, 255, 255, 15);
    margin: 3px 8px;
}
QMenu::icon {
    padding-left: 8px;
}
)";

LuzumiDesktopMenu::LuzumiDesktopMenu(QObject *parent, const QVariantList &args)
    : Plasma::ContainmentActions(parent, args)
{
}

QString LuzumiDesktopMenu::desktopPath() const
{
    return QStandardPaths::writableLocation(QStandardPaths::DesktopLocation);
}

QList<QAction *> LuzumiDesktopMenu::contextualActions()
{
    showStyledMenu();
    return {};
}

void LuzumiDesktopMenu::showStyledMenu()
{
    auto *menu = new QMenu();
    menu->setAttribute(Qt::WA_DeleteOnClose);
    menu->setStyleSheet(QString::fromUtf8(MENU_STYLE));
    menu->setWindowFlag(Qt::FramelessWindowHint);

    QString desktop = desktopPath();

    // ── Section: Luzumi ──
    auto *headerLuzumi = menu->addAction(QStringLiteral("  LUZUMI"));
    headerLuzumi->setEnabled(false);

    menu->addAction(Icons::folder(), QStringLiteral("New Folder"), [desktop]() {
        QDir dir(desktop);
        QString defaultName = QStringLiteral("New Folder");
        if (dir.exists(defaultName)) {
            int i = 2;
            while (dir.exists(QStringLiteral("New Folder (%1)").arg(i))) i++;
            defaultName = QStringLiteral("New Folder (%1)").arg(i);
        }
        bool ok = false;
        QString name = QInputDialog::getText(nullptr, QStringLiteral("New Folder"),
            QStringLiteral("Folder name:"), QLineEdit::Normal, defaultName, &ok);
        if (!ok || name.trimmed().isEmpty()) return;
        name = name.trimmed();
        if (dir.exists(name)) return;
        dir.mkdir(name);
    });

    menu->addAction(Icons::file(), QStringLiteral("New File"), [desktop]() {
        QDir dir(desktop);
        QString defaultName = QStringLiteral("untitled.txt");
        if (dir.exists(defaultName)) {
            int i = 2;
            while (dir.exists(QStringLiteral("untitled (%1).txt").arg(i))) i++;
            defaultName = QStringLiteral("untitled (%1).txt").arg(i);
        }
        bool ok = false;
        QString name = QInputDialog::getText(nullptr, QStringLiteral("New File"),
            QStringLiteral("File name:"), QLineEdit::Normal, defaultName, &ok);
        if (!ok || name.trimmed().isEmpty()) return;
        name = name.trimmed();
        if (dir.exists(name)) return;
        QFile file(dir.filePath(name));
        (void)file.open(QIODevice::WriteOnly);
        file.close();
    });

    menu->addAction(Icons::paste(), QStringLiteral("Paste"), [desktop]() {
        const QClipboard *clipboard = QApplication::clipboard();
        const QMimeData *mimeData = clipboard->mimeData();
        if (!mimeData || !mimeData->hasUrls()) return;

        for (const QUrl &url : mimeData->urls()) {
            if (!url.isLocalFile()) continue;
            const QString srcPath = url.toLocalFile();
            const QFileInfo srcInfo(srcPath);
            if (!srcInfo.exists()) continue;

            QString destName = srcInfo.fileName();
            QString destPath = desktop + QStringLiteral("/") + destName;

            // Handle name conflicts
            if (QFile::exists(destPath)) {
                const QString baseName = srcInfo.completeBaseName();
                const QString suffix = srcInfo.suffix();
                int i = 2;
                do {
                    if (suffix.isEmpty()) {
                        destPath = desktop + QStringLiteral("/%1 (%2)").arg(baseName).arg(i);
                    } else {
                        destPath = desktop + QStringLiteral("/%1 (%2).%3").arg(baseName).arg(i).arg(suffix);
                    }
                    i++;
                } while (QFile::exists(destPath));
            }

            if (srcInfo.isDir()) {
                QProcess::startDetached(QStringLiteral("cp"), {QStringLiteral("-r"), srcPath, destPath});
            } else {
                QFile::copy(srcPath, destPath);
            }
        }
    });

    menu->addSeparator();

    menu->addAction(Icons::terminal(), QStringLiteral("Open Terminal Here"), [desktop]() {
        QString term = qEnvironmentVariable("TERMINAL");
        if (term.isEmpty()) {
            for (const auto &t : {QStringLiteral("kitty"), QStringLiteral("konsole"),
                                   QStringLiteral("alacritty"), QStringLiteral("gnome-terminal")}) {
                if (!QStandardPaths::findExecutable(t).isEmpty()) {
                    term = t;
                    break;
                }
            }
        }
        if (!term.isEmpty()) {
            QStringList args;
            if (term.contains(QStringLiteral("kitty"))) {
                args = {QStringLiteral("--directory"), desktop};
            } else if (term.contains(QStringLiteral("alacritty"))) {
                args = {QStringLiteral("--working-directory"), desktop};
            } else if (term.contains(QStringLiteral("gnome-terminal"))) {
                args = {QStringLiteral("--working-directory"), desktop};
            } else {
                // konsole and others
                args = {QStringLiteral("--workdir"), desktop};
            }
            QProcess::startDetached(term, args);
        }
    });

    menu->addAction(Icons::app(), QStringLiteral("Open Luzumi"), [desktop]() {
        QString luzumi = QStandardPaths::findExecutable(QStringLiteral("luzumi"));
        if (luzumi.isEmpty()) {
            luzumi = QDir::homePath() + QStringLiteral("/.local/bin/luzumi");
        }
        QProcess::startDetached(luzumi, {desktop});
    });

    menu->addSeparator();

    // ── Section: Desktop ──
    auto *headerKDE = menu->addAction(QStringLiteral("  DESKTOP"));
    headerKDE->setEnabled(false);

    menu->addAction(Icons::lock(), QStringLiteral("Lock Screen"), []() {
        QDBusMessage msg = QDBusMessage::createMethodCall(
            QStringLiteral("org.freedesktop.ScreenSaver"),
            QStringLiteral("/ScreenSaver"),
            QStringLiteral("org.freedesktop.ScreenSaver"),
            QStringLiteral("Lock"));
        QDBusConnection::sessionBus().asyncCall(msg);
    });

    menu->addAction(Icons::logout(), QStringLiteral("Show Logout Screen"), []() {
        QDBusMessage msg = QDBusMessage::createMethodCall(
            QStringLiteral("org.kde.LogoutPrompt"),
            QStringLiteral("/LogoutPrompt"),
            QStringLiteral("org.kde.LogoutPrompt"),
            QStringLiteral("promptAll"));
        QDBusConnection::sessionBus().asyncCall(msg);
    });

    menu->addAction(Icons::wallpaper(), QStringLiteral("Wallpaper & Desktop Settings"), []() {
        QString cmd = QStandardPaths::findExecutable(QStringLiteral("plasma-open-settings"));
        if (!cmd.isEmpty()) {
            QProcess::startDetached(cmd, {QStringLiteral("kcm_wallpaper")});
        } else {
            QProcess::startDetached(QStringLiteral("kcmshell6"), {QStringLiteral("kcm_lookandfeel")});
        }
    });

    menu->addAction(Icons::display(), QStringLiteral("Display Configuration"), []() {
        QString cmd = QStandardPaths::findExecutable(QStringLiteral("plasma-open-settings"));
        if (!cmd.isEmpty()) {
            QProcess::startDetached(cmd, {QStringLiteral("kcm_kscreen")});
        } else {
            QProcess::startDetached(QStringLiteral("kcmshell6"), {QStringLiteral("kcm_kscreen")});
        }
    });

    menu->popup(QCursor::pos());
}

#include "LuzumiDesktopMenu.moc"
