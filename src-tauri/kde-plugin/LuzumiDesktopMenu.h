#ifndef LUZUMI_DESKTOP_MENU_H
#define LUZUMI_DESKTOP_MENU_H

#include <Plasma/ContainmentActions>
#include <QAction>

class QMenu;

class LuzumiDesktopMenu : public Plasma::ContainmentActions
{
    Q_OBJECT

public:
    explicit LuzumiDesktopMenu(QObject *parent, const QVariantList &args);
    ~LuzumiDesktopMenu() override = default;

    QList<QAction *> contextualActions() override;

private:
    void showStyledMenu();
    QString desktopPath() const;

    QAction *m_triggerAction = nullptr;
};

#endif // LUZUMI_DESKTOP_MENU_H
