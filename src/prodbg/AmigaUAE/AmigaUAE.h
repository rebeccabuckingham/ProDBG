#pragma once

#include <QObject>
#include <QProcess>
#include <QString>

#include "Config/AmigaUAEConfig.h"

class QTemporaryDir;

namespace prodbg {

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class AmigaUAE : public QObject
{
    Q_OBJECT

public:
    AmigaUAE(QObject* parent);
    ~AmigaUAE();

    bool openFile();
    void runExecutable(const QString& filename);
    bool validateSettings();
    void launchUAE();
    void killProcess();

    // TODO: Structure this better

    QProcess* m_uaeProcess;
    uint16_t m_setFileId;
    uint16_t m_setHddPathId;
    QString m_uaeExe;
    QString m_config;
    QString m_cmdLineArgs;
    QString m_dh0Path;
    QString m_fileToRun;
    QString m_localExeToRun;
    QString m_romPath;
    bool m_copyFiles;
    bool m_skipUAELaunch;
    AmigaUAEConfig::ConfigMode m_configMode;

private:
    Q_SLOT void started();
    Q_SLOT void errorOccurred(QProcess::ProcessError error);

    QTemporaryDir* m_tempDir;

    void readSettings();

    bool m_running = false;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
}