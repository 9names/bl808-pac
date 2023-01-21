#[doc = "Register `dbg_cfg1` reader"]
pub struct R(crate::R<DBG_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dbg_cfg1` writer"]
pub struct W(crate::W<DBG_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBG_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DBG_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBG_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_dbg_lh_ctrl` reader - "]
pub type REG_DBG_LH_CTRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `reg_dbg_lh_ctrl` writer - "]
pub type REG_DBG_LH_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBG_CFG1_SPEC, u32, u32, 30, O>;
#[doc = "Field `reg_dbg_lh_sel` reader - "]
pub type REG_DBG_LH_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_dbg_lh_sel` writer - "]
pub type REG_DBG_LH_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBG_CFG1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn reg_dbg_lh_ctrl(&self) -> REG_DBG_LH_CTRL_R {
        REG_DBG_LH_CTRL_R::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_dbg_lh_sel(&self) -> REG_DBG_LH_SEL_R {
        REG_DBG_LH_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    #[must_use]
    pub fn reg_dbg_lh_ctrl(&mut self) -> REG_DBG_LH_CTRL_W<0> {
        REG_DBG_LH_CTRL_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_dbg_lh_sel(&mut self) -> REG_DBG_LH_SEL_W<30> {
        REG_DBG_LH_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dbg_cfg1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_cfg1](index.html) module"]
pub struct DBG_CFG1_SPEC;
impl crate::RegisterSpec for DBG_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_cfg1::R](R) reader structure"]
impl crate::Readable for DBG_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbg_cfg1::W](W) writer structure"]
impl crate::Writable for DBG_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dbg_cfg1 to value 0"]
impl crate::Resettable for DBG_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
